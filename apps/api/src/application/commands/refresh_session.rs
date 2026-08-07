use std::sync::Arc;

use crate::domain::ports::{RefreshTokenRepository, TokenIssuer, UserRepository};
use crate::domain::user::AuthError;
use crate::infrastructure::auth::secrets::{split_refresh_token, verify_secret};

use super::google_sign_in::{issue_session, IssuedSession};

const REUSE_GRACE_MS: i64 = 30_000;

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

    let rotated_within_grace = record.replaced_by.is_some()
      && record
        .revoked_at_ms
        .is_some_and(|rotated_at| now_ms.saturating_sub(rotated_at) <= REUSE_GRACE_MS);

    if record.is_revoked() && !rotated_within_grace {
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

    if !record.is_revoked() {
      let replacement_id = split_refresh_token(&session.refresh_token)
        .map(|(id, _)| id.to_string())
        .unwrap_or_default();

      self.refresh_repo.mark_rotated(token_id, &replacement_id, now_ms).await?;
    }

    Ok(session)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::domain::ports::RepositoryError;
  use crate::domain::user::{AccessClaims, GoogleIdentity, RefreshTokenRecord, User};
  use crate::infrastructure::auth::secrets::{compose_refresh_token, generate_secret, hash_secret};
  use async_trait::async_trait;
  use std::sync::Mutex;

  const NOW: i64 = 1_700_000_000_000;
  const USER: &str = "user-1";

  struct FakeUsers;

  #[async_trait]
  impl UserRepository for FakeUsers {
    async fn upsert_by_google_sub(&self, _identity: &GoogleIdentity, _now_ms: i64) -> Result<User, RepositoryError> {
      unreachable!("not used by refresh")
    }

    async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, RepositoryError> {
      Ok(Some(User {
        id: user_id.to_string(),
        google_sub: "sub".into(),
        email: "a@b.c".into(),
        email_verified: true,
        name: None,
        picture: None,
        created_at_ms: 0,
        updated_at_ms: 0,
        last_login_at_ms: 0,
      }))
    }
  }

  struct FakeTokens;

  impl TokenIssuer for FakeTokens {
    fn issue_access(&self, user_id: &str, _now_ms: i64) -> Result<String, AuthError> {
      Ok(format!("access-{user_id}"))
    }

    fn verify_access(&self, _token: &str) -> Result<AccessClaims, AuthError> {
      unreachable!("not used by refresh")
    }

    fn access_ttl_secs(&self) -> i64 {
      900
    }
  }

  #[derive(Default)]
  struct FakeRefresh {
    records: Mutex<Vec<RefreshTokenRecord>>,
    revoke_all_calls: Mutex<Vec<String>>,
    rotated_calls: Mutex<Vec<String>>,
  }

  #[async_trait]
  impl RefreshTokenRepository for FakeRefresh {
    async fn insert(&self, record: &RefreshTokenRecord) -> Result<(), RepositoryError> {
      self.records.lock().unwrap().push(record.clone());
      Ok(())
    }

    async fn find(&self, token_id: &str) -> Result<Option<RefreshTokenRecord>, RepositoryError> {
      Ok(self.records.lock().unwrap().iter().find(|r| r.id == token_id).cloned())
    }

    async fn mark_rotated(&self, token_id: &str, replaced_by: &str, now_ms: i64) -> Result<(), RepositoryError> {
      self.rotated_calls.lock().unwrap().push(token_id.to_string());
      let mut records = self.records.lock().unwrap();
      if let Some(record) = records.iter_mut().find(|r| r.id == token_id) {
        record.revoked_at_ms = Some(now_ms);
        record.replaced_by = Some(replaced_by.to_string());
      }
      Ok(())
    }

    async fn revoke(&self, _token_id: &str, _now_ms: i64) -> Result<(), RepositoryError> {
      Ok(())
    }

    async fn revoke_all_for_user(&self, user_id: &str, _now_ms: i64) -> Result<(), RepositoryError> {
      self.revoke_all_calls.lock().unwrap().push(user_id.to_string());
      Ok(())
    }
  }

  fn seed(revoked_at_ms: Option<i64>, replaced_by: Option<&str>) -> (Arc<FakeRefresh>, String) {
    let repo = Arc::new(FakeRefresh::default());
    let token_id = "token-1".to_string();
    let secret = generate_secret();

    repo.records.lock().unwrap().push(RefreshTokenRecord {
      id: token_id.clone(),
      user_id: USER.to_string(),
      token_hash: hash_secret(&secret),
      device_id: Some("device-1".into()),
      created_at_ms: NOW - 60_000,
      expires_at_ms: NOW + 60_000,
      last_used_at_ms: NOW - 60_000,
      revoked_at_ms,
      replaced_by: replaced_by.map(|id| id.to_string()),
    });

    (repo, compose_refresh_token(&token_id, &secret))
  }

  fn handler(repo: Arc<FakeRefresh>) -> RefreshSessionHandler {
    RefreshSessionHandler {
      user_repo: Arc::new(FakeUsers),
      refresh_repo: repo,
      token_issuer: Arc::new(FakeTokens),
      refresh_ttl_secs: 2_592_000,
    }
  }

  #[tokio::test]
  async fn live_token_rotates_normally() {
    let (repo, token) = seed(None, None);
    let result = handler(repo.clone())
      .handle(RefreshSessionCommand { refresh_token: token }, NOW)
      .await;

    assert!(result.is_ok());
    assert_eq!(repo.rotated_calls.lock().unwrap().as_slice(), ["token-1"]);
    assert!(repo.revoke_all_calls.lock().unwrap().is_empty());
  }

  #[tokio::test]
  async fn token_rotated_within_grace_issues_a_session_without_revoking_everything() {
    let (repo, token) = seed(Some(NOW - 5_000), Some("token-2"));
    let result = handler(repo.clone())
      .handle(RefreshSessionCommand { refresh_token: token }, NOW)
      .await;

    assert!(result.is_ok());
    assert!(repo.revoke_all_calls.lock().unwrap().is_empty());
  }

  #[tokio::test]
  async fn grace_refresh_does_not_extend_the_window() {
    let (repo, token) = seed(Some(NOW - 5_000), Some("token-2"));
    let _ = handler(repo.clone())
      .handle(RefreshSessionCommand { refresh_token: token }, NOW)
      .await;

    assert!(repo.rotated_calls.lock().unwrap().is_empty());
    let records = repo.records.lock().unwrap();
    let original = records.iter().find(|r| r.id == "token-1").unwrap();
    assert_eq!(original.revoked_at_ms, Some(NOW - 5_000));
    assert_eq!(original.replaced_by.as_deref(), Some("token-2"));
  }

  #[tokio::test]
  async fn token_rotated_outside_grace_revokes_every_session() {
    let (repo, token) = seed(Some(NOW - 60_000), Some("token-2"));
    let result = handler(repo.clone())
      .handle(RefreshSessionCommand { refresh_token: token }, NOW)
      .await;

    assert!(matches!(result, Err(AuthError::Revoked)));
    assert_eq!(repo.revoke_all_calls.lock().unwrap().as_slice(), [USER]);
  }

  #[tokio::test]
  async fn explicitly_revoked_token_revokes_every_session() {
    let (repo, token) = seed(Some(NOW - 1_000), None);
    let result = handler(repo.clone())
      .handle(RefreshSessionCommand { refresh_token: token }, NOW)
      .await;

    assert!(matches!(result, Err(AuthError::Revoked)));
    assert_eq!(repo.revoke_all_calls.lock().unwrap().as_slice(), [USER]);
  }
}
