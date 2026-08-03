use async_trait::async_trait;
use mongodb::Database;

use crate::infrastructure::mongo::auth_ticket_repository::AuthTicketsCollConf;

use super::Migration;

pub struct AuthTickets;

#[async_trait]
impl Migration for AuthTickets {
  fn id(&self) -> &'static str {
    "0004_auth_tickets"
  }

  async fn up(&self, db: &Database) -> mongodb::error::Result<()> {
    mongodm::sync_indexes::<AuthTicketsCollConf>(db).await
  }
}
