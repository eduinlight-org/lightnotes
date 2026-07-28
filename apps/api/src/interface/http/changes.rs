use std::convert::Infallible;
use std::time::Duration;

use axum::extract::{Query, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::Json;
use futures_util::{Stream, StreamExt};
use serde::Deserialize;
use sync_dto::{AcceptedChange, PullChangesResponse, PushChangesRequest, PushChangesResponse, QueuedChange, RejectedChange, ServerChange};

use crate::application::commands::push_changes::PushChangesCommand;
use crate::application::queries::pull_changes::PullChangesQuery;
use crate::application::queries::stream_changes::{StreamChangesQuery, StreamItem};
use crate::domain::change::Change;
use crate::domain::ports::StoredChange;

use super::state::AppState;

fn now_ms() -> i64 {
  std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .expect("system clock before unix epoch")
    .as_millis() as i64
}

fn to_domain_change(queued: QueuedChange) -> Change {
  Change {
    change_id: queued.change_id,
    device_id: queued.device_id,
    entity: queued.entity,
    entity_id: queued.entity_id,
    op: queued.op,
    payload: queued.payload,
    client_updated_at_ms: queued.client_updated_at_ms,
  }
}

pub async fn push(State(state): State<AppState>, Json(request): Json<PushChangesRequest>) -> Json<PushChangesResponse> {
  let changes = request.changes.into_iter().map(to_domain_change).collect();
  let outcome = state.push_handler.handle(PushChangesCommand { changes }, now_ms()).await;

  Json(PushChangesResponse {
    accepted: outcome
      .accepted
      .into_iter()
      .map(|accepted| AcceptedChange { change_id: accepted.change_id, seq: accepted.seq })
      .collect(),
    rejected: outcome
      .rejected
      .into_iter()
      .map(|rejected| RejectedChange { change_id: rejected.change_id, reason: rejected.reason })
      .collect(),
  })
}

#[derive(Debug, Deserialize)]
pub struct PullParams {
  #[serde(default)]
  since: i64,
}

pub async fn pull(State(state): State<AppState>, Query(params): Query<PullParams>) -> Json<PullChangesResponse> {
  let outcome = state
    .pull_handler
    .handle(PullChangesQuery { since: params.since })
    .await
    .expect("pull query failed");

  Json(PullChangesResponse {
    changes: outcome.changes.into_iter().map(to_server_change).collect(),
    cursor: outcome.cursor,
  })
}

fn to_server_change(stored: StoredChange) -> ServerChange {
  ServerChange {
    seq: stored.seq,
    change_id: stored.change_id,
    device_id: stored.device_id,
    entity: stored.entity,
    entity_id: stored.entity_id,
    op: stored.op,
    payload: stored.payload,
    server_applied_at_ms: stored.server_applied_at_ms,
  }
}

pub async fn stream(
  State(state): State<AppState>,
  Query(params): Query<PullParams>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
  let items = state
    .stream_handler
    .handle(StreamChangesQuery { since: params.since })
    .await
    .expect("stream query failed");

  let event_stream = items.map(|item| {
    Ok(match item {
      StreamItem::Change(stored) => {
        let server_change = to_server_change(*stored);
        Event::default()
          .event("change")
          .id(server_change.seq.to_string())
          .json_data(&server_change)
          .unwrap_or_else(|_| Event::default())
      }
      StreamItem::CaughtUp { cursor } => Event::default().event("caught-up").data(cursor.to_string()),
    })
  });

  Sse::new(event_stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}
