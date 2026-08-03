use std::sync::Arc;

use crate::domain::ports::{GoogleIdentityVerifier, RefreshTokenRepository, TokenIssuer, UserRepository};
use crate::domain::user::{AuthError, RefreshTokenRecord, User};
use crate::infrastructure::auth::secrets::{compose_refresh_token, generate_secret, hash_secret};

pub struct GoogleSignInCommand {
  pub id_token: String,
  pub device_id: Option<String>,
}

pub struct IssuedSession {
  pub user: User,
  pub access_token: String,
  pub refresh_token: String,
  pub expires_in_secs: i64,
}

pub struct GoogleSignInHandler {
  pub verifier: Arc<dyn GoogleIdentityVerifier>,
  pub user_repo: Arc<dyn UserRepository>,
  pub refresh_repo: Arc<dyn RefreshTokenRepository>,
  pub token_issuer: Arc<dyn TokenIssuer>,
  pub refresh_ttl_secs: i64,
}

impl GoogleSignInHandler {
  pub async fn handle(&self, command: GoogleSignInCommand, now_ms: i64) -> Result<IssuedSession, AuthError> {
    let identity = self.verifier.verify(&command.id_token).await?;
    let user = self.user_repo.upsert_by_google_sub(&identity, now_ms).await?;

    issue_session(
      self.refresh_repo.as_ref(),
      self.token_issuer.as_ref(),
      user,
      command.device_id,
      self.refresh_ttl_secs,
      now_ms,
    )
    .await
  }
}

pub async fn issue_session(
  refresh_repo: &dyn RefreshTokenRepository,
  token_issuer: &dyn TokenIssuer,
  user: User,
  device_id: Option<String>,
  refresh_ttl_secs: i64,
  now_ms: i64,
) -> Result<IssuedSession, AuthError> {
  let access_token = token_issuer.issue_access(&user.id, now_ms)?;

  let token_id = uuid::Uuid::new_v4().to_string();
  let secret = generate_secret();

  let record = RefreshTokenRecord {
    id: token_id.clone(),
    user_id: user.id.clone(),
    token_hash: hash_secret(&secret),
    device_id,
    created_at_ms: now_ms,
    expires_at_ms: now_ms + refresh_ttl_secs * 1000,
    last_used_at_ms: now_ms,
    revoked_at_ms: None,
    replaced_by: None,
  };

  refresh_repo.insert(&record).await?;

  Ok(IssuedSession {
    user,
    access_token,
    refresh_token: compose_refresh_token(&token_id, &secret),
    expires_in_secs: token_issuer.access_ttl_secs(),
  })
}
