use async_trait::async_trait;
use mongodb::Database;

use crate::infrastructure::mongo::change_repository::ChangesCollConf;

use super::Migration;

pub struct InitIndexes;

#[async_trait]
impl Migration for InitIndexes {
  fn id(&self) -> &'static str {
    "0001_init_indexes"
  }

  async fn up(&self, db: &Database) -> mongodb::error::Result<()> {
    mongodm::sync_indexes::<ChangesCollConf>(db).await
  }
}
