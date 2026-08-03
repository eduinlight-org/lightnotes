use std::convert::Infallible;
use std::time::Duration;

use axum::extract::{Query, State};
use axum::http::StatusCode;
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

use super::auth_user::AuthUser;
use super::state::AppState;

fn now_ms() -> i64 {
  std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .expect("system clock before unix epoch")
    .as_millis() as i64
}

fn to_domain_change(user_id: &str, queued: QueuedChange) -> Change {
  Change {
    user_id: user_id.to_string(),
    change_id: queued.change_id,
    device_id: queued.device_id,
    entity: queued.entity,
    entity_id: queued.entity_id,
    op: queued.op,
    payload: queued.payload,
    client_updated_at_ms: queued.client_updated_at_ms,
  }
}

pub async fn push(State(state): State<AppState>, user: AuthUser, Json(request): Json<PushChangesRequest>) -> Json<PushChangesResponse> {
  let changes = request
    .changes
    .into_iter()
    .map(|queued| to_domain_change(&user.user_id, queued))
    .collect();
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

pub async fn pull(
  State(state): State<AppState>,
  user: AuthUser,
  Query(params): Query<PullParams>,
) -> Result<Json<PullChangesResponse>, StatusCode> {
  let outcome = state
    .pull_handler
    .handle(PullChangesQuery {
      user_id: user.user_id,
      since: params.since,
    })
    .await
    .map_err(|err| {
      tracing::error!("pull query failed: {err}");
      StatusCode::INTERNAL_SERVER_ERROR
    })?;

  Ok(Json(PullChangesResponse {
    changes: outcome.changes.into_iter().map(to_server_change).collect(),
    cursor: outcome.cursor,
  }))
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
  user: AuthUser,
  Query(params): Query<PullParams>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
  let items = state
    .stream_handler
    .handle(StreamChangesQuery {
      user_id: user.user_id,
      since: params.since,
    })
    .await
    .map_err(|err| {
      tracing::error!("stream query failed: {err}");
      StatusCode::INTERNAL_SERVER_ERROR
    })?;

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

  Ok(Sse::new(event_stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15))))
}
