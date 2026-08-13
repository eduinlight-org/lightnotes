use axum::extract::State;
use axum::http::StatusCode;

use super::state::AppState;

pub async fn check() -> StatusCode {
  StatusCode::OK
}

pub async fn ready(State(state): State<AppState>) -> StatusCode {
  match state.health_probe.ping().await {
    Ok(()) => StatusCode::OK,
    Err(error) => {
      tracing::warn!(%error, "readiness probe failed");
      StatusCode::SERVICE_UNAVAILABLE
    }
  }
}
