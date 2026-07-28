use std::sync::Arc;

use crate::domain::change::Change;
use crate::domain::ports::RepositoryError;

use super::apply_change::{ApplyChangeCommand, ApplyChangeHandler};

pub struct PushChangesCommand {
  pub changes: Vec<Change>,
}

pub struct Accepted {
  pub change_id: String,
  pub seq: i64,
}

pub struct Rejected {
  pub change_id: String,
  pub reason: String,
}

pub struct PushOutcome {
  pub accepted: Vec<Accepted>,
  pub rejected: Vec<Rejected>,
}

pub struct PushChangesHandler {
  pub apply_handler: Arc<ApplyChangeHandler>,
}

impl PushChangesHandler {
  pub async fn handle(&self, command: PushChangesCommand, server_applied_at_ms: i64) -> PushOutcome {
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for change in command.changes {
      let change_id = change.change_id.clone();
      match self.apply_handler.handle(ApplyChangeCommand { change }, server_applied_at_ms).await {
        Ok(result) => accepted.push(Accepted { change_id: result.change_id, seq: result.seq }),
        Err(RepositoryError::Backend(reason)) => rejected.push(Rejected { change_id, reason }),
      }
    }

    PushOutcome { accepted, rejected }
  }
}
