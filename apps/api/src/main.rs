mod application;
mod domain;
mod infrastructure;
mod interface;

use std::sync::Arc;

use tokio::net::TcpListener;

use application::commands::apply_change::ApplyChangeHandler;
use application::commands::push_changes::PushChangesHandler;
use application::queries::pull_changes::PullChangesHandler;
use application::queries::stream_changes::StreamChangesHandler;
use infrastructure::config::Config;
use infrastructure::mongo::change_repository::MongoChangeRepository;
use infrastructure::mongo::read_model_repository::MongoReadModelRepository;
use infrastructure::notifier::BroadcastChangeNotifier;
use interface::http::router::router;
use interface::http::state::AppState;

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  let config = Config::from_env();

  let _logging_guard = infrastructure::logging::init(config.is_production());

  let db = infrastructure::db::connect(&config.mongodb_uri).await;

  infrastructure::migrations::run_pending(&db).await;

  let change_repo: Arc<dyn domain::ports::ChangeRepository> = Arc::new(MongoChangeRepository::new(db.clone()));
  let read_model_repo: Arc<dyn domain::ports::ReadModelRepository> = Arc::new(MongoReadModelRepository::new(db.clone()));
  let notifier: Arc<dyn domain::ports::ChangeNotifier> = Arc::new(BroadcastChangeNotifier::new());

  let apply_handler = Arc::new(ApplyChangeHandler { change_repo: change_repo.clone(), read_model_repo, notifier: notifier.clone() });
  let push_handler = Arc::new(PushChangesHandler { apply_handler });
  let pull_handler = Arc::new(PullChangesHandler { change_repo: change_repo.clone() });
  let stream_handler = Arc::new(StreamChangesHandler { change_repo, notifier });

  let state = AppState { push_handler, pull_handler, stream_handler };

  let listener = TcpListener::bind(("0.0.0.0", config.api_port)).await.expect("failed to bind API_PORT");

  tracing::info!("api listening on port {}", config.api_port);

  axum::serve(listener, router(state)).await.expect("server error");
}
