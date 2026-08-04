use std::sync::Arc;

use crate::domain::ports::{RefreshTokenRepository, TokenIssuer, UserRepository};
use crate::domain::user::AuthError;
use crate::infrastructure::auth::secrets::{split_refresh_token, verify_secret};

use super::google_sign_in::{issue_session, IssuedSession};

pub struct RefreshSessionCommand {
  pub refresh_token: String,
}

pub struct RefreshSessionHandler {
  pub user_repo: Arc<dyn UserRepository>,
  pub refresh_repo: Arc<dyn RefreshTokenRepository>,
  pub token_issuer: Arc<dyn TokenIssuer>,
  pub refresh_ttl_secs: i64,
}

impl RefreshSessionHandler {
  pub async fn handle(&self, command: RefreshSessionCommand, now_ms: i64) -> Result<IssuedSession, AuthError> {
    let (token_id, secret) = split_refresh_token(&command.refresh_token).ok_or(AuthError::InvalidToken)?;

    let record = self.refresh_repo.find(token_id).await?.ok_or(AuthError::InvalidToken)?;

    if !verify_secret(secret, &record.token_hash) {
      return Err(AuthError::InvalidToken);
    }

    if record.is_revoked() {
      self.refresh_repo.revoke_all_for_user(&record.user_id, now_ms).await?;
      return Err(AuthError::Revoked);
    }

    if record.is_expired(now_ms) {
      return Err(AuthError::Expired);
    }

    let user = self
      .user_repo
      .find_by_id(&record.user_id)
      .await?
      .ok_or(AuthError::InvalidToken)?;

    let session = issue_session(
      self.refresh_repo.as_ref(),
      self.token_issuer.as_ref(),
      user,
      record.device_id.clone(),
      self.refresh_ttl_secs,
      now_ms,
    )
    .await?;

    let replacement_id = split_refresh_token(&session.refresh_token)
      .map(|(id, _)| id.to_string())
      .unwrap_or_default();

    self.refresh_repo.mark_rotated(token_id, &replacement_id, now_ms).await?;

    Ok(session)
  }
}
