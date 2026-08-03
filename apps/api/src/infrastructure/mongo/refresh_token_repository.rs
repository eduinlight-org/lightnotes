use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::Database;
use mongodm::{CollectionConfig, Index, Indexes, Model, ToRepository};
use serde::{Deserialize, Serialize};

use crate::domain::ports::{RefreshTokenRepository, RepositoryError};
use crate::domain::user::RefreshTokenRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RefreshTokenDoc {
  #[serde(rename = "_id")]
  id: String,
  user_id: String,
  token_hash: String,
  device_id: Option<String>,
  created_at_ms: i64,
  expires_at_ms: i64,
  last_used_at_ms: i64,
  revoked_at_ms: Option<i64>,
  replaced_by: Option<String>,
}

impl From<RefreshTokenDoc> for RefreshTokenRecord {
  fn from(value: RefreshTokenDoc) -> Self {
    RefreshTokenRecord {
      id: value.id,
      user_id: value.user_id,
      token_hash: value.token_hash,
      device_id: value.device_id,
      created_at_ms: value.created_at_ms,
      expires_at_ms: value.expires_at_ms,
      last_used_at_ms: value.last_used_at_ms,
      revoked_at_ms: value.revoked_at_ms,
      replaced_by: value.replaced_by,
    }
  }
}

impl From<&RefreshTokenRecord> for RefreshTokenDoc {
  fn from(value: &RefreshTokenRecord) -> Self {
    RefreshTokenDoc {
      id: value.id.clone(),
      user_id: value.user_id.clone(),
      token_hash: value.token_hash.clone(),
      device_id: value.device_id.clone(),
      created_at_ms: value.created_at_ms,
      expires_at_ms: value.expires_at_ms,
      last_used_at_ms: value.last_used_at_ms,
      revoked_at_ms: value.revoked_at_ms,
      replaced_by: value.replaced_by.clone(),
    }
  }
}

pub struct RefreshTokensCollConf;

impl CollectionConfig for RefreshTokensCollConf {
  fn collection_name() -> &'static str {
    "refresh_tokens"
  }

  fn indexes() -> Indexes {
    Indexes::new().with(Index::new("user_id")).with(Index::new("expires_at_ms"))
  }
}

impl Model for RefreshTokenDoc {
  type CollConf = RefreshTokensCollConf;
}

pub struct MongoRefreshTokenRepository {
  db: Database,
}

impl MongoRefreshTokenRepository {
  pub fn new(db: Database) -> Self {
    Self { db }
  }
}

fn backend_err(err: impl std::fmt::Display) -> RepositoryError {
  RepositoryError::Backend(err.to_string())
}

#[async_trait]
impl RefreshTokenRepository for MongoRefreshTokenRepository {
  async fn insert(&self, record: &RefreshTokenRecord) -> Result<(), RepositoryError> {
    let repo = self.db.repository::<RefreshTokenDoc>();
    let doc: RefreshTokenDoc = record.into();

    repo.insert_one(&doc).await.map_err(backend_err)?;

    Ok(())
  }

  async fn find(&self, token_id: &str) -> Result<Option<RefreshTokenRecord>, RepositoryError> {
    let repo = self.db.repository::<RefreshTokenDoc>();

    let found = repo.find_one(doc! { "_id": token_id }).await.map_err(backend_err)?;

    Ok(found.map(Into::into))
  }

  async fn mark_rotated(&self, token_id: &str, replaced_by: &str, now_ms: i64) -> Result<(), RepositoryError> {
    let repo = self.db.repository::<RefreshTokenDoc>();

    repo
      .update_one(
        doc! { "_id": token_id },
        doc! { "$set": { "revoked_at_ms": now_ms, "replaced_by": replaced_by, "last_used_at_ms": now_ms } },
      )
      .await
      .map_err(backend_err)?;

    Ok(())
  }

  async fn revoke(&self, token_id: &str, now_ms: i64) -> Result<(), RepositoryError> {
    let repo = self.db.repository::<RefreshTokenDoc>();

    repo
      .update_one(doc! { "_id": token_id }, doc! { "$set": { "revoked_at_ms": now_ms } })
      .await
      .map_err(backend_err)?;

    Ok(())
  }

  async fn revoke_all_for_user(&self, user_id: &str, now_ms: i64) -> Result<(), RepositoryError> {
    let repo = self.db.repository::<RefreshTokenDoc>();

    repo
      .update_many(
        doc! { "user_id": user_id, "revoked_at_ms": { "$eq": null } },
        doc! { "$set": { "revoked_at_ms": now_ms } },
      )
      .await
      .map_err(backend_err)?;

    Ok(())
  }
}
