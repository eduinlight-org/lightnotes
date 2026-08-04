use std::sync::Arc;

use crate::domain::ports::UserRepository;
use crate::domain::user::{AuthError, User};

pub struct CurrentUserQuery {
  pub user_id: String,
}

pub struct CurrentUserHandler {
  pub user_repo: Arc<dyn UserRepository>,
}

impl CurrentUserHandler {
  pub async fn handle(&self, query: CurrentUserQuery) -> Result<User, AuthError> {
    self
      .user_repo
      .find_by_id(&query.user_id)
      .await?
      .ok_or(AuthError::InvalidToken)
  }
}
