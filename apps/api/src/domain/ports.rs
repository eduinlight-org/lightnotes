use std::pin::Pin;

use async_trait::async_trait;
use futures_util::Stream;
use sync_dto::{ChangeOp, ChangePayload, EntityKind, FolderDto, NoteDto, TagDto};

use super::change::Change;
use super::user::{AccessClaims, AuthError, GoogleIdentity, NativeAuthTicket, RefreshTokenRecord, User};

#[derive(Debug, Clone, PartialEq)]
pub struct StoredChange {
  pub user_id: String,
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
  async fn reserve_seq(&self, user_id: &str) -> Result<i64, RepositoryError>;
  async fn insert_if_new(&self, change: &Change, seq: i64, server_applied_at_ms: i64) -> Result<InsertOutcome, RepositoryError>;
  async fn list_since(&self, user_id: &str, since: i64) -> Result<Vec<StoredChange>, RepositoryError>;
}

#[async_trait]
pub trait HealthProbe: Send + Sync {
  async fn ping(&self) -> Result<(), RepositoryError>;
}

pub trait ChangeNotifier: Send + Sync {
  fn notify(&self, change: StoredChange);
  fn subscribe(&self, user_id: &str) -> Pin<Box<dyn Stream<Item = StoredChange> + Send>>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
  async fn upsert_by_google_sub(&self, identity: &GoogleIdentity, now_ms: i64) -> Result<User, RepositoryError>;
  async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, RepositoryError>;
}

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
  async fn insert(&self, record: &RefreshTokenRecord) -> Result<(), RepositoryError>;
  async fn find(&self, token_id: &str) -> Result<Option<RefreshTokenRecord>, RepositoryError>;
  async fn mark_rotated(&self, token_id: &str, replaced_by: &str, now_ms: i64) -> Result<(), RepositoryError>;
  async fn revoke(&self, token_id: &str, now_ms: i64) -> Result<(), RepositoryError>;
  async fn revoke_all_for_user(&self, user_id: &str, now_ms: i64) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait AuthTicketRepository: Send + Sync {
  async fn insert(&self, ticket: &NativeAuthTicket) -> Result<(), RepositoryError>;
  async fn find_by_state(&self, state: &str) -> Result<Option<NativeAuthTicket>, RepositoryError>;
  async fn complete(&self, ticket_id: &str, user_id: &str, access_token: &str, refresh_token: &str, expires_in_secs: i64)
    -> Result<(), RepositoryError>;
  async fn fail(&self, ticket_id: &str) -> Result<(), RepositoryError>;
  async fn take(&self, ticket_id: &str) -> Result<Option<NativeAuthTicket>, RepositoryError>;
}

#[async_trait]
pub trait GoogleCodeExchanger: Send + Sync {
  async fn authorize_url(&self, state: &str, pkce_challenge: &str) -> String;
  async fn exchange_code(&self, code: &str, pkce_verifier: &str) -> Result<String, AuthError>;
}

pub trait TokenIssuer: Send + Sync {
  fn issue_access(&self, user_id: &str, now_ms: i64) -> Result<String, AuthError>;
  fn verify_access(&self, token: &str) -> Result<AccessClaims, AuthError>;
  fn access_ttl_secs(&self) -> i64;
}

#[async_trait]
pub trait GoogleIdentityVerifier: Send + Sync {
  async fn verify(&self, id_token: &str) -> Result<GoogleIdentity, AuthError>;
}

#[async_trait]
pub trait ReadModelRepository: Send + Sync {
  async fn existing_updated_at_ms(&self, user_id: &str, entity: EntityKind, entity_id: &str) -> Result<Option<i64>, RepositoryError>;
  async fn upsert_note(&self, user_id: &str, note: &NoteDto) -> Result<(), RepositoryError>;
  async fn upsert_folder(&self, user_id: &str, folder: &FolderDto) -> Result<(), RepositoryError>;
  async fn upsert_tag(&self, user_id: &str, tag: &TagDto) -> Result<(), RepositoryError>;
  async fn delete_note(&self, user_id: &str, id: &str) -> Result<(), RepositoryError>;
  async fn delete_folder(&self, user_id: &str, id: &str) -> Result<(), RepositoryError>;
  async fn delete_tag(&self, user_id: &str, id: &str) -> Result<(), RepositoryError>;
}
