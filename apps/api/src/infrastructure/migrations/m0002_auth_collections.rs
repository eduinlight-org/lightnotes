use async_trait::async_trait;
use mongodb::Database;

use crate::infrastructure::mongo::refresh_token_repository::RefreshTokensCollConf;
use crate::infrastructure::mongo::user_repository::UsersCollConf;

use super::Migration;

pub struct AuthCollections;

#[async_trait]
impl Migration for AuthCollections {
  fn id(&self) -> &'static str {
    "0002_auth_collections"
  }

  async fn up(&self, db: &Database) -> mongodb::error::Result<()> {
    mongodm::sync_indexes::<UsersCollConf>(db).await?;
    mongodm::sync_indexes::<RefreshTokensCollConf>(db).await
  }
}
