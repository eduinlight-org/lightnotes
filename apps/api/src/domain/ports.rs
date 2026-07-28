use std::pin::Pin;

use async_trait::async_trait;
use futures_util::Stream;
use sync_dto::{ChangeOp, ChangePayload, EntityKind, FolderDto, NoteDto, TagDto};

use super::change::Change;

#[derive(Debug, Clone, PartialEq)]
pub struct StoredChange {
  pub seq: i64,
  pub change_id: String,
  pub device_id: String,
  pub entity: EntityKind,
  pub entity_id: String,
  pub op: ChangeOp,
  pub payload: Option<ChangePayload>,
  pub client_updated_at_ms: i64,
  pub server_applied_at_ms: i64,
}

pub enum InsertOutcome {
  Inserted(StoredChange),
  AlreadyExists(StoredChange),
}

#[derive(Debug, Clone)]
pub enum RepositoryError {
  Backend(String),
}

impl std::fmt::Display for RepositoryError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      RepositoryError::Backend(reason) => write!(f, "repository error: {reason}"),
    }
  }
}

impl std::error::Error for RepositoryError {}

#[async_trait]
pub trait ChangeRepository: Send + Sync {
  async fn reserve_seq(&self) -> Result<i64, RepositoryError>;
  async fn insert_if_new(&self, change: &Change, seq: i64, server_applied_at_ms: i64) -> Result<InsertOutcome, RepositoryError>;
  async fn list_since(&self, since: i64) -> Result<Vec<StoredChange>, RepositoryError>;
}

pub trait ChangeNotifier: Send + Sync {
  fn notify(&self, change: StoredChange);
  fn subscribe(&self) -> Pin<Box<dyn Stream<Item = StoredChange> + Send>>;
}

#[async_trait]
pub trait ReadModelRepository: Send + Sync {
  async fn existing_updated_at_ms(&self, entity: EntityKind, entity_id: &str) -> Result<Option<i64>, RepositoryError>;
  async fn upsert_note(&self, note: &NoteDto) -> Result<(), RepositoryError>;
  async fn upsert_folder(&self, folder: &FolderDto) -> Result<(), RepositoryError>;
  async fn upsert_tag(&self, tag: &TagDto) -> Result<(), RepositoryError>;
  async fn delete_note(&self, id: &str) -> Result<(), RepositoryError>;
  async fn delete_folder(&self, id: &str) -> Result<(), RepositoryError>;
  async fn delete_tag(&self, id: &str) -> Result<(), RepositoryError>;
}
