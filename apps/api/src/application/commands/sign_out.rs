use std::sync::Arc;

use crate::domain::ports::RefreshTokenRepository;
use crate::domain::user::AuthError;
use crate::infrastructure::auth::secrets::{split_refresh_token, verify_secret};

pub struct SignOutCommand {
  pub refresh_token: String,
}

pub struct SignOutHandler {
  pub refresh_repo: Arc<dyn RefreshTokenRepository>,
}

impl SignOutHandler {
  pub async fn handle(&self, command: SignOutCommand, now_ms: i64) -> Result<(), AuthError> {
    let Some((token_id, secret)) = split_refresh_token(&command.refresh_token) else {
      return Ok(());
    };

    let Some(record) = self.refresh_repo.find(token_id).await? else {
      return Ok(());
    };

    if !verify_secret(secret, &record.token_hash) {
      return Ok(());
    }

    self.refresh_repo.revoke(token_id, now_ms).await?;

    Ok(())
  }
}
