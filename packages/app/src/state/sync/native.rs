use std::time::Duration;

use api_sdk::SseChangeEvent;
use dioxus::prelude::*;
use futures_util::StreamExt;
use store_sdk::{use_synced_store, StoreConfig, StoreHandle};
use sync_dto::QueuedChange;

use super::dto::{api_base_url, compute_next_id, diff_folders, diff_notes, diff_tags, folder_from_dto, merge_server_changes, note_from_dto, tag_from_dto};
use crate::state::auth::{use_auth, AuthState, AuthStatus};
use crate::state::boot::use_boot;
use crate::state::notes::{Folder, Note, NotesStore, SyncStatus, Tag};
use crate::state::preferences::use_persisted_preferences;

const BASE_BACKOFF_MS: u64 = 1000;
const MAX_BACKOFF_MS: u64 = 30_000;
const OUTBOUND_DEBOUNCE_MS: u64 = 600;

async fn apply_outbound(handle: StoreHandle, user_id: String, changes: Vec<QueuedChange>) {
  for change in &changes {
    handle.apply(&user_id, change).await;
    handle.enqueue_outbound(&user_id, change).await;
  }
}

fn persist_tokens(handle: &StoreHandle, mut auth: AuthState) {
  let api = handle.api();

  if let Some(rotated) = api.take_rotated_tokens() {
    auth.set_tokens(rotated);
  }

  if api.take_session_expired() && *auth.status.peek() == AuthStatus::SignedIn {
    auth.sign_out();
  }
}

pub fn use_synced_notes() -> NotesStore {
  let mut store = use_context_provider(NotesStore::empty);
  use_persisted_preferences(store);

  let auth = use_auth();
  let boot = use_boot();

  let mut offline = use_signal(|| false);
  use_effect(move || {
    offline.set(store.sync() == SyncStatus::Offline);
  });

  let mut active_user = use_signal(|| None::<String>);
  use_effect(move || {
    active_user.set(auth.user().map(|user| user.id));
  });

  let handle = use_synced_store(StoreConfig::new(api_base_url()), offline, active_user);
  use_context_provider(|| handle.clone());

  let mut hydrated = boot.store_ready;
  let session_ready = boot.session_ready;
  let mut device_id = use_signal(String::new);
  let mut last_synced_notes = use_signal(Vec::<Note>::new);
  let mut last_synced_folders = use_signal(Vec::<Folder>::new);
  let mut last_synced_tags = use_signal(Vec::<Tag>::new);
  let mut sync_generation = use_signal(|| 0u64);
  let mut stream_task = use_signal(|| None::<dioxus::core::Task>);

  let identity_handle = handle.clone();
  use_hook(move || {
    spawn(async move {
      device_id.set(identity_handle.device_id().await);
    });
  });

  let reset_handle = handle.clone();
  use_effect(move || {
    if !session_ready() {
      return;
    }

    let _auth_generation = auth.generation();
    let signed_in = auth.is_signed_in();
    let tokens = auth.tokens.peek().clone();
    let user_id = auth.user.peek().as_ref().map(|user| user.id.clone());

    if let Some(task) = stream_task.write().take() {
      task.cancel();
    }

    *sync_generation.write() += 1;
    hydrated.set(false);

    store.clear_synced_entities();
    last_synced_notes.set(Vec::new());
    last_synced_folders.set(Vec::new());
    last_synced_tags.set(Vec::new());

    reset_handle.api().set_tokens(tokens);

    let Some(user_id) = user_id else {
      hydrated.set(true);
      return;
    };

    store.set_user(user_id.clone());

    let handle = reset_handle.clone();
    let task = spawn(async move {
      let local_snapshot = handle.load_snapshot(&user_id).await;
      let notes: Vec<Note> = local_snapshot.notes.into_iter().map(|dto| note_from_dto(&user_id, dto)).collect();
      let folders: Vec<Folder> = local_snapshot.folders.into_iter().map(|dto| folder_from_dto(&user_id, dto)).collect();
      let tags: Vec<Tag> = local_snapshot.tags.into_iter().map(|dto| tag_from_dto(&user_id, dto)).collect();

      let next_id = compute_next_id(&notes, &folders, &tags);
      let mut persisted = store.snapshot();
      persisted.notes = notes;
      persisted.folders = folders;
      persisted.tags = tags;
      persisted.next_id = next_id;
      store.restore(persisted);

      let baseline = store.snapshot();
      last_synced_notes.set(baseline.notes);
      last_synced_folders.set(baseline.folders);
      last_synced_tags.set(baseline.tags);

      hydrated.set(true);

      if !signed_in {
        return;
      }

      let device = handle.device_id().await;
      let mut since = handle.cursor(&user_id).await;
      let mut backoff_ms = BASE_BACKOFF_MS;

      loop {
        if *offline.peek() {
          tokio::time::sleep(Duration::from_millis(BASE_BACKOFF_MS)).await;
          continue;
        }

        let mut stream = handle.subscribe_changes(since);

        while let Some(event) = stream.next().await {
          backoff_ms = BASE_BACKOFF_MS;

          match event {
            SseChangeEvent::Change(change) => {
              since = since.max(change.seq);
              let is_self_echo = change.device_id == device;

              let queued = QueuedChange {
                change_id: change.change_id.clone(),
                device_id: device.clone(),
                entity: change.entity,
                entity_id: change.entity_id.clone(),
                op: change.op,
                payload: change.payload.clone(),
                client_updated_at_ms: change.server_applied_at_ms,
                enqueued_at_ms: change.server_applied_at_ms,
              };
              handle.apply(&user_id, &queued).await;
              handle.set_cursor(&user_id, since).await;

              if !is_self_echo {
                let mut snapshot = store.snapshot();
                merge_server_changes(&user_id, &mut snapshot, vec![*change]);
                store.restore(snapshot);

                let baseline = store.snapshot();
                last_synced_notes.set(baseline.notes);
                last_synced_folders.set(baseline.folders);
                last_synced_tags.set(baseline.tags);
              }
            }
            SseChangeEvent::CaughtUp { cursor } => {
              since = since.max(cursor);
              handle.set_cursor(&user_id, since).await;
            }
          }
        }

        drop(stream);
        persist_tokens(&handle, auth);

        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
        backoff_ms = (backoff_ms * 2).min(MAX_BACKOFF_MS);
      }
    });

    stream_task.set(Some(task));
  });

  let outbound_handle = handle.clone();
  use_effect(move || {
    let is_hydrated = hydrated();
    let is_signed_in = auth.is_signed_in();
    let _ = store.snapshot();

    let Some(user_id) = auth.user.peek().as_ref().map(|user| user.id.clone()) else {
      return;
    };

    if !is_hydrated || !is_signed_in {
      return;
    }

    let generation = {
      let mut generation = sync_generation.write();
      *generation += 1;
      *generation
    };

    let handle = outbound_handle.clone();

    spawn(async move {
      tokio::time::sleep(Duration::from_millis(OUTBOUND_DEBOUNCE_MS)).await;

      if *sync_generation.peek() != generation {
        return;
      }

      let device = device_id();
      let current = store.snapshot();
      let previous_notes = last_synced_notes.peek().clone();
      let previous_folders = last_synced_folders.peek().clone();
      let previous_tags = last_synced_tags.peek().clone();

      let mut changes = diff_notes(&previous_notes, &current.notes, &device);
      changes.extend(diff_folders(&previous_folders, &current.folders, &device));
      changes.extend(diff_tags(&previous_tags, &current.tags, &device));

      last_synced_notes.set(current.notes);
      last_synced_folders.set(current.folders);
      last_synced_tags.set(current.tags);

      apply_outbound(handle, user_id, changes).await;
    });
  });

  store
}
