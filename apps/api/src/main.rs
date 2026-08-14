mod application;
mod domain;
mod infrastructure;
mod interface;

use std::sync::Arc;

use tokio::net::TcpListener;

use application::commands::apply_change::ApplyChangeHandler;
use application::commands::google_sign_in::GoogleSignInHandler;
use application::commands::native_auth::NativeAuthHandler;
use application::commands::push_changes::PushChangesHandler;
use application::commands::refresh_session::RefreshSessionHandler;
use application::commands::sign_out::SignOutHandler;
use application::queries::current_user::CurrentUserHandler;
use application::queries::pull_changes::PullChangesHandler;
use application::queries::stream_changes::StreamChangesHandler;
use infrastructure::auth::google::GoogleOauthVerifier;
use infrastructure::auth::google_code::GoogleCodeClient;
use infrastructure::auth::jwt::HmacTokenIssuer;
use infrastructure::mongo::auth_ticket_repository::MongoAuthTicketRepository;
use infrastructure::config::Config;
use infrastructure::mongo::change_repository::MongoChangeRepository;
use infrastructure::mongo::health_probe::MongoHealthProbe;
use infrastructure::mongo::read_model_repository::MongoReadModelRepository;
use infrastructure::mongo::refresh_token_repository::MongoRefreshTokenRepository;
use infrastructure::mongo::user_repository::MongoUserRepository;
use infrastructure::notifier::BroadcastChangeNotifier;
use interface::http::router::router;
use interface::http::state::AppState;

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  let config = Config::from_env();

  let telemetry = infrastructure::telemetry::init(&config);
  let metrics = Arc::new(infrastructure::telemetry::AppMetrics::new());

  let db = infrastructure::db::connect(&config.mongodb_uri, &metrics).await;

  infrastructure::migrations::run_pending(&db).await;

  let change_repo: Arc<dyn domain::ports::ChangeRepository> = Arc::new(MongoChangeRepository::new(db.clone()));
  let read_model_repo: Arc<dyn domain::ports::ReadModelRepository> = Arc::new(MongoReadModelRepository::new(db.clone()));
  let notifier: Arc<dyn domain::ports::ChangeNotifier> = Arc::new(BroadcastChangeNotifier::new());

  let apply_handler = Arc::new(ApplyChangeHandler { change_repo: change_repo.clone(), read_model_repo, notifier: notifier.clone() });
  let push_handler = Arc::new(PushChangesHandler { apply_handler });
  let pull_handler = Arc::new(PullChangesHandler { change_repo: change_repo.clone() });
  let stream_handler = Arc::new(StreamChangesHandler { change_repo, notifier });

  let user_repo: Arc<dyn domain::ports::UserRepository> = Arc::new(MongoUserRepository::new(db.clone()));
  let refresh_repo: Arc<dyn domain::ports::RefreshTokenRepository> = Arc::new(MongoRefreshTokenRepository::new(db.clone()));
  let ticket_repo: Arc<dyn domain::ports::AuthTicketRepository> = Arc::new(MongoAuthTicketRepository::new(db.clone()));
  let health_probe: Arc<dyn domain::ports::HealthProbe> = Arc::new(MongoHealthProbe::new(db));
  let token_issuer: Arc<dyn domain::ports::TokenIssuer> =
    Arc::new(HmacTokenIssuer::new(&config.jwt_secret, config.access_token_ttl_secs));
  let google_verifier: Arc<dyn domain::ports::GoogleIdentityVerifier> = Arc::new(GoogleOauthVerifier::new(&config.google_client_ids));
  let code_exchanger: Arc<dyn domain::ports::GoogleCodeExchanger> = Arc::new(GoogleCodeClient::new(
    config.google_client_ids[0].clone(),
    config.google_client_secret.clone(),
    config.google_redirect_uri.clone(),
  ));

  let native_auth_handler = Arc::new(NativeAuthHandler {
    ticket_repo,
    exchanger: code_exchanger,
    verifier: google_verifier.clone(),
    user_repo: user_repo.clone(),
    refresh_repo: refresh_repo.clone(),
    token_issuer: token_issuer.clone(),
    refresh_ttl_secs: config.refresh_token_ttl_secs,
  });

  let google_sign_in_handler = Arc::new(GoogleSignInHandler {
    verifier: google_verifier,
    user_repo: user_repo.clone(),
    refresh_repo: refresh_repo.clone(),
    token_issuer: token_issuer.clone(),
    refresh_ttl_secs: config.refresh_token_ttl_secs,
  });
  let refresh_session_handler = Arc::new(RefreshSessionHandler {
    user_repo: user_repo.clone(),
    refresh_repo: refresh_repo.clone(),
    token_issuer: token_issuer.clone(),
    refresh_ttl_secs: config.refresh_token_ttl_secs,
  });
  let sign_out_handler = Arc::new(SignOutHandler { refresh_repo });
  let current_user_handler = Arc::new(CurrentUserHandler { user_repo });

  let state = AppState {
    push_handler,
    pull_handler,
    stream_handler,
    google_sign_in_handler,
    native_auth_handler,
    refresh_session_handler,
    sign_out_handler,
    current_user_handler,
    token_issuer,
    google_client_id: config.google_client_ids[0].clone(),
    health_probe,
    metrics,
  };

  let listener = TcpListener::bind(("0.0.0.0", config.api_port)).await.expect("failed to bind API_PORT");

  tracing::info!("api listening on port {}", config.api_port);

  axum::serve(listener, router(state))
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("server error");

  tokio::task::spawn_blocking(move || telemetry.shutdown()).await.expect("failed to flush telemetry");
}

async fn shutdown_signal() {
  let interrupt = async {
    tokio::signal::ctrl_c().await.expect("failed to install SIGINT handler");
  };

  #[cfg(unix)]
  let terminate = async {
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
      .expect("failed to install SIGTERM handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
    _ = interrupt => {},
    _ = terminate => {},
  }

  tracing::info!("shutdown signal received, draining connections");
}
