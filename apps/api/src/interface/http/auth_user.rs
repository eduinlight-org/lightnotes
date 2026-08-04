use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::domain::user::AuthError;

use super::state::AppState;

pub struct AuthUser {
  pub user_id: String,
}

pub struct AuthRejection;

impl IntoResponse for AuthRejection {
  fn into_response(self) -> Response {
    (
      StatusCode::UNAUTHORIZED,
      [(header::WWW_AUTHENTICATE, "Bearer")],
      Json(serde_json::json!({ "error": "unauthorized" })),
    )
      .into_response()
  }
}

impl From<AuthError> for AuthRejection {
  fn from(_: AuthError) -> Self {
    AuthRejection
  }
}

impl FromRequestParts<AppState> for AuthUser {
  type Rejection = AuthRejection;

  async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
    let header_value = parts
      .headers
      .get(header::AUTHORIZATION)
      .and_then(|value| value.to_str().ok())
      .ok_or(AuthRejection)?;

    let token = header_value.strip_prefix("Bearer ").ok_or(AuthRejection)?;

    let claims = state.token_issuer.verify_access(token.trim())?;

    Ok(AuthUser { user_id: claims.sub })
  }
}
