use axum::handler::Handler;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use axum::Router;
use axum_helmet::{Helmet, HelmetLayer};
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

use super::auth;
use super::changes;
use super::health;
use super::state::AppState;
use super::telemetry::{self, AuthRoute};

pub fn router(state: AppState) -> Router {
  let helmet_layer: HelmetLayer = Helmet::default().try_into().expect("failed to build helmet layer");
  let metrics = state.metrics.clone();

  let requests = || from_fn_with_state(metrics.clone(), telemetry::record_requests);
  let pushed_changes = || from_fn_with_state(metrics.clone(), telemetry::record_pushed_changes);
  let open_streams = || from_fn_with_state(metrics.clone(), telemetry::track_open_stream);
  let auth_attempts = |method| {
    from_fn_with_state(
      AuthRoute {
        metrics: metrics.clone(),
        method,
      },
      telemetry::record_auth_attempt,
    )
  };

  Router::new()
    .route("/healthz", get(health::check))
    .route("/readyz", get(health::ready))
    .route(
      "/api/v1/changes",
      post(changes::push.layer(pushed_changes())).get(changes::pull),
    )
    .route("/api/v1/changes/stream", get(changes::stream).layer(open_streams()))
    .route("/api/v1/auth/config", get(auth::config))
    .route(
      "/api/v1/auth/google",
      post(auth::google_sign_in).layer(auth_attempts("google")),
    )
    .route("/api/v1/auth/refresh", post(auth::refresh).layer(auth_attempts("refresh")))
    .route("/api/v1/auth/logout", post(auth::logout).layer(auth_attempts("logout")))
    .route("/api/v1/auth/me", get(auth::me))
    .route(
      "/api/v1/auth/native/start",
      post(auth::native_start).layer(auth_attempts("native_start")),
    )
    .route(
      "/api/v1/auth/native/callback",
      get(auth::native_callback).layer(auth_attempts("native_callback")),
    )
    .route(
      "/api/v1/auth/native/poll",
      post(auth::native_poll).layer(auth_attempts("native_poll")),
    )
    .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024))
    .layer(CorsLayer::permissive())
    .layer(helmet_layer)
    .layer(requests())
    .layer(TraceLayer::new_for_http().make_span_with(telemetry::make_span))
    .with_state(state)
}
