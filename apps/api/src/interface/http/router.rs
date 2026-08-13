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
use super::telemetry;

pub fn router(state: AppState) -> Router {
  let helmet_layer: HelmetLayer = Helmet::default().try_into().expect("failed to build helmet layer");
  let metrics = state.metrics.clone();

  Router::new()
    .route("/healthz", get(health::check))
    .route("/readyz", get(health::ready))
    .route("/api/v1/changes", post(changes::push).get(changes::pull))
    .route("/api/v1/changes/stream", get(changes::stream))
    .route("/api/v1/auth/config", get(auth::config))
    .route("/api/v1/auth/google", post(auth::google_sign_in))
    .route("/api/v1/auth/refresh", post(auth::refresh))
    .route("/api/v1/auth/logout", post(auth::logout))
    .route("/api/v1/auth/me", get(auth::me))
    .route("/api/v1/auth/native/start", post(auth::native_start))
    .route("/api/v1/auth/native/callback", get(auth::native_callback))
    .route("/api/v1/auth/native/poll", post(auth::native_poll))
    .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024))
    .layer(CorsLayer::permissive())
    .layer(helmet_layer)
    .layer(axum::middleware::from_fn_with_state(metrics, telemetry::record_metrics))
    .layer(TraceLayer::new_for_http().make_span_with(telemetry::make_span))
    .with_state(state)
}
