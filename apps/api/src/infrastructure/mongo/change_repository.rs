use async_trait::async_trait;
use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::ReturnDocument;
use mongodb::{Collection, Database};
use mongodm::{CollectionConfig, Index, IndexOption, Indexes, Model, ToRepository};
use serde::{Deserialize, Serialize};
use sync_dto::{ChangeOp, ChangePayload, EntityKind};

use crate::domain::change::Change;
use crate::domain::ports::{ChangeRepository, InsertOutcome, RepositoryError, StoredChange};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredChangeDoc {
  user_id: String,
  change_id: String,
  seq: i64,
  device_id: String,
  entity: EntityKind,
  entity_id: String,
  op: ChangeOp,
  payload: Option<ChangePayload>,
  client_updated_at_ms: i64,
  server_applied_at_ms: i64,
}

impl From<StoredChangeDoc> for StoredChange {
  fn from(value: StoredChangeDoc) -> Self {
    StoredChange {
      user_id: value.user_id,
      seq: value.seq,
      change_id: value.change_id,
      device_id: value.device_id,
      entity: value.entity,
      entity_id: value.entity_id,
      op: value.op,
      payload: value.payload,
      client_updated_at_ms: value.client_updated_at_ms,
      server_applied_at_ms: value.server_applied_at_ms,
    }
  }
}

pub struct ChangesCollConf;

impl CollectionConfig for ChangesCollConf {
  fn collection_name() -> &'static str {
    "changes"
  }

  fn indexes() -> Indexes {
    Indexes::new()
      .with(Index::new("user_id").with_key("change_id").with_option(IndexOption::Unique))
      .with(Index::new("user_id").with_key("seq").with_option(IndexOption::Unique))
  }
}

impl Model for StoredChangeDoc {
  type CollConf = ChangesCollConf;
}

pub struct MongoChangeRepository {
  db: Database,
  counters: Collection<mongodb::bson::Document>,
}

impl MongoChangeRepository {
  pub fn new(db: Database) -> Self {
    let counters = db.collection("counters");
    Self { db, counters }
  }
}

fn backend_err(err: impl std::fmt::Display) -> RepositoryError {
  RepositoryError::Backend(err.to_string())
}

fn is_duplicate_key_error(err: &mongodb::error::Error) -> bool {
  err.to_string().contains("E11000")
}

#[async_trait]
impl ChangeRepository for MongoChangeRepository {
  async fn reserve_seq(&self, user_id: &str) -> Result<i64, RepositoryError> {
    let updated = self
      .counters
      .find_one_and_update(
        doc! { "_id": format!("changes_seq:{user_id}") },
        doc! { "$inc": { "value": 1_i64 } },
      )
      .upsert(true)
      .return_document(ReturnDocument::After)
      .await
      .map_err(backend_err)?;

    let doc = updated.ok_or_else(|| RepositoryError::Backend("counter upsert returned no document".into()))?;
    doc.get_i64("value").map_err(backend_err)
  }

  async fn insert_if_new(&self, change: &Change, seq: i64, server_applied_at_ms: i64) -> Result<InsertOutcome, RepositoryError> {
    let repo = self.db.repository::<StoredChangeDoc>();

    let scope = doc! { "user_id": &change.user_id, "change_id": &change.change_id };

    if let Some(existing) = repo.find_one(scope.clone()).await.map_err(backend_err)? {
      return Ok(InsertOutcome::AlreadyExists(existing.into()));
    }

    let stored = StoredChangeDoc {
      user_id: change.user_id.clone(),
      change_id: change.change_id.clone(),
      seq,
      device_id: change.device_id.clone(),
      entity: change.entity,
      entity_id: change.entity_id.clone(),
      op: change.op,
      payload: change.payload.clone(),
      client_updated_at_ms: change.client_updated_at_ms,
      server_applied_at_ms,
    };

    match repo.insert_one(&stored).await {
      Ok(_) => Ok(InsertOutcome::Inserted(stored.into())),
      Err(err) if is_duplicate_key_error(&err) => {
        let existing = repo
          .find_one(scope)
          .await
          .map_err(backend_err)?
          .ok_or_else(|| RepositoryError::Backend("duplicate change vanished".into()))?;
        Ok(InsertOutcome::AlreadyExists(existing.into()))
      }
      Err(err) => Err(backend_err(err)),
    }
  }

  async fn list_since(&self, user_id: &str, since: i64) -> Result<Vec<StoredChange>, RepositoryError> {
    let repo = self.db.repository::<StoredChangeDoc>();

    let cursor = repo
      .find(doc! { "user_id": user_id, "seq": { "$gt": since } })
      .sort(doc! { "seq": 1_i32 })
      .await
      .map_err(backend_err)?;

    let docs: Vec<StoredChangeDoc> = cursor.try_collect().await.map_err(backend_err)?;
    Ok(docs.into_iter().map(Into::into).collect())
  }
}
