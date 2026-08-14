use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::Database;

use crate::domain::ports::{HealthProbe, RepositoryError};

pub struct MongoHealthProbe {
  db: Database,
}

impl MongoHealthProbe {
  pub fn new(db: Database) -> Self {
    Self { db }
  }
}

#[async_trait]
impl HealthProbe for MongoHealthProbe {
  async fn ping(&self) -> Result<(), RepositoryError> {
    self
      .db
      .run_command(doc! { "ping": 1 })
      .await
      .map(|_| ())
      .map_err(|error| RepositoryError::Backend(error.to_string()))
  }
}
