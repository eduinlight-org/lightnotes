use axum::routing::{get, post};
use axum::Router;
use axum_helmet::{Helmet, HelmetLayer};
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

use super::changes;
use super::health;
use super::state::AppState;

pub fn router(state: AppState) -> Router {
  let helmet_layer: HelmetLayer = Helmet::default().try_into().expect("failed to build helmet layer");

  Router::new()
    .route("/healthz", get(health::check))
    .route("/api/v1/changes", post(changes::push).get(changes::pull))
    .route("/api/v1/changes/stream", get(changes::stream))
    .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024))
    .layer(CorsLayer::permissive())
    .layer(helmet_layer)
    .layer(TraceLayer::new_for_http())
    .with_state(state)
}
