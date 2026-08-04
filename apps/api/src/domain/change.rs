use sync_dto::{ChangeOp, ChangePayload, EntityKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Change {
  pub user_id: String,
  pub change_id: String,
  pub device_id: String,
  pub entity: EntityKind,
  pub entity_id: String,
  pub op: ChangeOp,
  pub payload: Option<ChangePayload>,
  pub client_updated_at_ms: i64,
}

pub fn should_apply(existing_updated_at_ms: Option<i64>, incoming_updated_at_ms: i64) -> bool {
  sync_dto::is_newer_or_equal(existing_updated_at_ms, incoming_updated_at_ms)
}
