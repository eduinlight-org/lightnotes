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

pub fn router(state: AppState) -> Router {
  let helmet_layer: HelmetLayer = Helmet::default().try_into().expect("failed to build helmet layer");

  Router::new()
    .route("/healthz", get(health::check))
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
    .layer(TraceLayer::new_for_http())
    .with_state(state)
}
