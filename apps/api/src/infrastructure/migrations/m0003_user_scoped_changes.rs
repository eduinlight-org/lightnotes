use async_trait::async_trait;
use mongodb::bson::{doc, Document};
use mongodb::Database;

use crate::infrastructure::mongo::change_repository::ChangesCollConf;

use super::Migration;

pub struct UserScopedChanges;

#[async_trait]
impl Migration for UserScopedChanges {
  fn id(&self) -> &'static str {
    "0003_user_scoped_changes"
  }

  async fn up(&self, db: &Database) -> mongodb::error::Result<()> {
    mongodm::sync_indexes::<ChangesCollConf>(db).await?;

    for name in ["changes", "notes", "folders", "tags"] {
      db.collection::<Document>(name).drop().await?;
    }

    db.collection::<Document>("counters")
      .delete_one(doc! { "_id": "changes_seq" })
      .await?;

    Ok(())
  }
}
