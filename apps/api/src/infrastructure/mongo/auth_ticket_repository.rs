use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::Database;
use mongodm::{CollectionConfig, Index, IndexOption, Indexes, Model, ToRepository};
use serde::{Deserialize, Serialize};

use crate::domain::ports::{AuthTicketRepository, RepositoryError};
use crate::domain::user::{NativeAuthTicket, TicketStatus};

const PENDING: &str = "pending";
const COMPLETE: &str = "complete";
const FAILED: &str = "failed";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuthTicketDoc {
  #[serde(rename = "_id")]
  id: String,
  state: String,
  pkce_verifier: String,
  status: String,
  user_id: Option<String>,
  access_token: Option<String>,
  refresh_token: Option<String>,
  expires_in_secs: Option<i64>,
  created_at_ms: i64,
}

impl From<AuthTicketDoc> for NativeAuthTicket {
  fn from(value: AuthTicketDoc) -> Self {
    let status = match value.status.as_str() {
      COMPLETE => TicketStatus::Complete,
      FAILED => TicketStatus::Failed,
      _ => TicketStatus::Pending,
    };

    NativeAuthTicket {
      id: value.id,
      state: value.state,
      pkce_verifier: value.pkce_verifier,
      status,
      user_id: value.user_id,
      access_token: value.access_token,
      refresh_token: value.refresh_token,
      expires_in_secs: value.expires_in_secs,
      created_at_ms: value.created_at_ms,
    }
  }
}

pub struct AuthTicketsCollConf;

impl CollectionConfig for AuthTicketsCollConf {
  fn collection_name() -> &'static str {
    "auth_tickets"
  }

  fn indexes() -> Indexes {
    Indexes::new().with(Index::new("state").with_option(IndexOption::Unique))
  }
}

impl Model for AuthTicketDoc {
  type CollConf = AuthTicketsCollConf;
}

pub struct MongoAuthTicketRepository {
  db: Database,
}

impl MongoAuthTicketRepository {
  pub fn new(db: Database) -> Self {
    Self { db }
  }
}

fn backend_err(err: impl std::fmt::Display) -> RepositoryError {
  RepositoryError::Backend(err.to_string())
}

#[async_trait]
impl AuthTicketRepository for MongoAuthTicketRepository {
  async fn insert(&self, ticket: &NativeAuthTicket) -> Result<(), RepositoryError> {
    let repo = self.db.repository::<AuthTicketDoc>();

    let doc = AuthTicketDoc {
      id: ticket.id.clone(),
      state: ticket.state.clone(),
      pkce_verifier: ticket.pkce_verifier.clone(),
      status: PENDING.to_string(),
      user_id: None,
      access_token: None,
      refresh_token: None,
      expires_in_secs: None,
      created_at_ms: ticket.created_at_ms,
    };

    repo.insert_one(&doc).await.map_err(backend_err)?;

    Ok(())
  }

  async fn find_by_state(&self, state: &str) -> Result<Option<NativeAuthTicket>, RepositoryError> {
    let repo = self.db.repository::<AuthTicketDoc>();

    let found = repo.find_one(doc! { "state": state }).await.map_err(backend_err)?;

    Ok(found.map(Into::into))
  }

  async fn complete(
    &self,
    ticket_id: &str,
    user_id: &str,
    access_token: &str,
    refresh_token: &str,
    expires_in_secs: i64,
  ) -> Result<(), RepositoryError> {
    let repo = self.db.repository::<AuthTicketDoc>();

    repo
      .update_one(
        doc! { "_id": ticket_id },
        doc! { "$set": {
          "status": COMPLETE,
          "user_id": user_id,
          "access_token": access_token,
          "refresh_token": refresh_token,
          "expires_in_secs": expires_in_secs,
        }},
      )
      .await
      .map_err(backend_err)?;

    Ok(())
  }

  async fn fail(&self, ticket_id: &str) -> Result<(), RepositoryError> {
    let repo = self.db.repository::<AuthTicketDoc>();

    repo
      .update_one(doc! { "_id": ticket_id }, doc! { "$set": { "status": FAILED } })
      .await
      .map_err(backend_err)?;

    Ok(())
  }

  async fn take(&self, ticket_id: &str) -> Result<Option<NativeAuthTicket>, RepositoryError> {
    let repo = self.db.repository::<AuthTicketDoc>();

    let found = repo.find_one(doc! { "_id": ticket_id }).await.map_err(backend_err)?;

    let Some(doc) = found else {
      return Ok(None);
    };

    let ticket: NativeAuthTicket = doc.into();

    if ticket.status != TicketStatus::Pending {
      repo.delete_one(doc! { "_id": ticket_id }).await.map_err(backend_err)?;
    }

    Ok(Some(ticket))
  }
}
