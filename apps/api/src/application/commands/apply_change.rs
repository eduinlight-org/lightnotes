use std::sync::Arc;

use sync_dto::{ChangeOp, ChangePayload, EntityKind};

use crate::domain::change::{should_apply, Change};
use crate::domain::ports::{ChangeNotifier, ChangeRepository, InsertOutcome, ReadModelRepository, RepositoryError};

pub struct ApplyChangeCommand {
  pub change: Change,
}

pub struct AppliedChangeResult {
  pub change_id: String,
  pub seq: i64,
}

pub struct ApplyChangeHandler {
  pub change_repo: Arc<dyn ChangeRepository>,
  pub read_model_repo: Arc<dyn ReadModelRepository>,
  pub notifier: Arc<dyn ChangeNotifier>,
}

impl ApplyChangeHandler {
  pub async fn handle(&self, command: ApplyChangeCommand, server_applied_at_ms: i64) -> Result<AppliedChangeResult, RepositoryError> {
    let change = command.change;
    let seq = self.change_repo.reserve_seq().await?;

    let stored = match self.change_repo.insert_if_new(&change, seq, server_applied_at_ms).await? {
      InsertOutcome::AlreadyExists(stored) => {
        return Ok(AppliedChangeResult { change_id: stored.change_id, seq: stored.seq });
      }
      InsertOutcome::Inserted(stored) => stored,
    };

    if change.op == ChangeOp::Delete {
      self.delete_from_read_model(change.entity, &change.entity_id).await?;
    } else {
      let existing_ms = self.existing_updated_at_ms(change.entity, &change.entity_id).await?;
      if should_apply(existing_ms, change.client_updated_at_ms) {
        self.upsert_read_model(change.payload.as_ref()).await?;
      }
    }

    self.notifier.notify(stored.clone());

    Ok(AppliedChangeResult { change_id: stored.change_id, seq: stored.seq })
  }

  async fn existing_updated_at_ms(&self, entity: EntityKind, entity_id: &str) -> Result<Option<i64>, RepositoryError> {
    self.read_model_repo.existing_updated_at_ms(entity, entity_id).await
  }

  async fn upsert_read_model(&self, payload: Option<&ChangePayload>) -> Result<(), RepositoryError> {
    match payload {
      Some(ChangePayload::Note(note)) => self.read_model_repo.upsert_note(note).await,
      Some(ChangePayload::Folder(folder)) => self.read_model_repo.upsert_folder(folder).await,
      Some(ChangePayload::Tag(tag)) => self.read_model_repo.upsert_tag(tag).await,
      None => Ok(()),
    }
  }

  async fn delete_from_read_model(&self, entity: EntityKind, entity_id: &str) -> Result<(), RepositoryError> {
    match entity {
      EntityKind::Note => self.read_model_repo.delete_note(entity_id).await,
      EntityKind::Folder => self.read_model_repo.delete_folder(entity_id).await,
      EntityKind::Tag => self.read_model_repo.delete_tag(entity_id).await,
    }
  }
}
