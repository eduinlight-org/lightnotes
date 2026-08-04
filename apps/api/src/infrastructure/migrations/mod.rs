use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::Database;

mod m0001_init_indexes;
mod m0002_auth_collections;
mod m0003_user_scoped_changes;
mod m0004_auth_tickets;

#[async_trait]
pub trait Migration: Send + Sync {
  fn id(&self) -> &'static str;
  async fn up(&self, db: &Database) -> mongodb::error::Result<()>;
}

pub fn registry() -> Vec<Box<dyn Migration>> {
  vec![
    Box::new(m0001_init_indexes::InitIndexes),
    Box::new(m0002_auth_collections::AuthCollections),
    Box::new(m0003_user_scoped_changes::UserScopedChanges),
    Box::new(m0004_auth_tickets::AuthTickets),
  ]
}

pub async fn run_pending(db: &Database) {
  let applied = db.collection::<mongodb::bson::Document>("_migrations");

  for migration in registry() {
    let already_applied = applied
      .find_one(doc! { "_id": migration.id() })
      .await
      .expect("failed to query _migrations");

    if already_applied.is_some() {
      continue;
    }

    migration.up(db).await.unwrap_or_else(|err| panic!("migration {} failed: {err}", migration.id()));

    let applied_at_ms = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .expect("system clock before unix epoch")
      .as_millis() as i64;

    applied
      .insert_one(doc! { "_id": migration.id(), "applied_at_ms": applied_at_ms })
      .await
      .expect("failed to record applied migration");
  }
}
