use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::options::ReturnDocument;
use mongodb::Database;
use mongodm::{CollectionConfig, Index, IndexOption, Indexes, Model, ToRepository};
use serde::{Deserialize, Serialize};

use crate::domain::ports::{RepositoryError, UserRepository};
use crate::domain::user::{GoogleIdentity, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserDoc {
  #[serde(rename = "_id")]
  id: String,
  google_sub: String,
  email: String,
  email_verified: bool,
  name: Option<String>,
  picture: Option<String>,
  created_at_ms: i64,
  updated_at_ms: i64,
  last_login_at_ms: i64,
}

impl From<UserDoc> for User {
  fn from(value: UserDoc) -> Self {
    User {
      id: value.id,
      google_sub: value.google_sub,
      email: value.email,
      email_verified: value.email_verified,
      name: value.name,
      picture: value.picture,
      created_at_ms: value.created_at_ms,
      updated_at_ms: value.updated_at_ms,
      last_login_at_ms: value.last_login_at_ms,
    }
  }
}

pub struct UsersCollConf;

impl CollectionConfig for UsersCollConf {
  fn collection_name() -> &'static str {
    "users"
  }

  fn indexes() -> Indexes {
    Indexes::new()
      .with(Index::new("google_sub").with_option(IndexOption::Unique))
      .with(Index::new("email"))
  }
}

impl Model for UserDoc {
  type CollConf = UsersCollConf;
}

pub struct MongoUserRepository {
  db: Database,
}

impl MongoUserRepository {
  pub fn new(db: Database) -> Self {
    Self { db }
  }
}

fn backend_err(err: impl std::fmt::Display) -> RepositoryError {
  RepositoryError::Backend(err.to_string())
}

#[async_trait]
impl UserRepository for MongoUserRepository {
  async fn upsert_by_google_sub(&self, identity: &GoogleIdentity, now_ms: i64) -> Result<User, RepositoryError> {
    let repo = self.db.repository::<UserDoc>();

    let updated = repo
      .find_one_and_update(
        doc! { "google_sub": &identity.google_sub },
        doc! {
          "$set": {
            "email": &identity.email,
            "email_verified": identity.email_verified,
            "name": identity.name.as_deref(),
            "picture": identity.picture.as_deref(),
            "updated_at_ms": now_ms,
            "last_login_at_ms": now_ms,
          },
          "$setOnInsert": {
            "_id": uuid::Uuid::new_v4().to_string(),
            "google_sub": &identity.google_sub,
            "created_at_ms": now_ms,
          },
        },
      )
      .upsert(true)
      .return_document(ReturnDocument::After)
      .await
      .map_err(backend_err)?;

    let doc = updated.ok_or_else(|| RepositoryError::Backend("user upsert returned no document".into()))?;

    Ok(doc.into())
  }

  async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, RepositoryError> {
    let repo = self.db.repository::<UserDoc>();

    let found = repo.find_one(doc! { "_id": user_id }).await.map_err(backend_err)?;

    Ok(found.map(Into::into))
  }
}
