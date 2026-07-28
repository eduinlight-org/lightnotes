use std::time::Duration;

use api_sdk::SseChangeEvent;
use dioxus::prelude::*;
use futures_util::StreamExt;
use store_sdk::{use_synced_store, StoreConfig, StoreHandle};
use sync_dto::QueuedChange;

use super::dto::{api_base_url, compute_next_id, diff_folders, diff_notes, diff_tags, folder_from_dto, merge_server_changes, note_from_dto, tag_from_dto};
use crate::state::notes::{Folder, Note, NotesStore, SyncStatus, Tag};
use crate::state::preferences::use_persisted_preferences;

const BASE_BACKOFF_MS: u64 = 1000;
const MAX_BACKOFF_MS: u64 = 30_000;
const OUTBOUND_DEBOUNCE_MS: u64 = 600;

async fn apply_outbound(handle: StoreHandle, changes: Vec<QueuedChange>) {
  for change in &changes {
    handle.apply(change).await;
    handle.enqueue_outbound(change).await;
  }
}

pub fn use_synced_notes() -> NotesStore {
  let mut store = use_context_provider(NotesStore::seed);
  use_persisted_preferences(store);

  let mut offline = use_signal(|| false);
  use_effect(move || {
    offline.set(store.sync() == SyncStatus::Offline);
  });

  let handle = use_synced_store(StoreConfig::new(api_base_url()), offline);
  let mut loaded = use_signal(|| false);
  let mut device_id = use_signal(String::new);
  let mut last_synced_notes = use_signal(Vec::<Note>::new);
  let mut last_synced_folders = use_signal(Vec::<Folder>::new);
  let mut last_synced_tags = use_signal(Vec::<Tag>::new);
  let mut sync_generation = use_signal(|| 0u64);

  let hydrate_handle = handle.clone();
  use_effect(move || {
    let handle = hydrate_handle.clone();

    spawn(async move {
      let device = handle.device_id().await;
      device_id.set(device);

      let local_snapshot = handle.load_snapshot().await;
      let notes: Vec<Note> = local_snapshot.notes.into_iter().map(note_from_dto).collect();
      let folders: Vec<Folder> = local_snapshot.folders.into_iter().map(folder_from_dto).collect();
      let tags: Vec<Tag> = local_snapshot.tags.into_iter().map(tag_from_dto).collect();

      if !(notes.is_empty() && folders.is_empty() && tags.is_empty()) {
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
      }
    });
  });

  let stream_handle = handle.clone();
  use_hook(move || {
    spawn(async move {
      let device = stream_handle.device_id().await;
      let mut since = stream_handle.cursor().await;
      let mut backoff_ms = BASE_BACKOFF_MS;

      loop {
        if *offline.peek() {
          tokio::time::sleep(Duration::from_millis(BASE_BACKOFF_MS)).await;
          continue;
        }

        let mut stream = stream_handle.subscribe_changes(since);

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
              stream_handle.apply(&queued).await;
              stream_handle.set_cursor(since).await;

              if !is_self_echo {
                let mut snapshot = store.snapshot();
                merge_server_changes(&mut snapshot, vec![*change]);
                store.restore(snapshot);

                let baseline = store.snapshot();
                last_synced_notes.set(baseline.notes);
                last_synced_folders.set(baseline.folders);
                last_synced_tags.set(baseline.tags);
              }
            }
            SseChangeEvent::CaughtUp { cursor } => {
              since = since.max(cursor);
              stream_handle.set_cursor(since).await;

              if !*loaded.peek() {
                let baseline = store.snapshot();

                if baseline.notes.is_empty() && baseline.folders.is_empty() && baseline.tags.is_empty() {
                  last_synced_notes.set(Vec::new());
                  last_synced_folders.set(Vec::new());
                  last_synced_tags.set(Vec::new());
                } else {
                  last_synced_notes.set(baseline.notes);
                  last_synced_folders.set(baseline.folders);
                  last_synced_tags.set(baseline.tags);
                }

                loaded.set(true);
              }
            }
          }
        }

        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
        backoff_ms = (backoff_ms * 2).min(MAX_BACKOFF_MS);
      }
    });
  });

  use_effect(move || {
    let is_loaded = loaded();
    let _ = store.snapshot();

    if !is_loaded {
      return;
    }

    let generation = {
      let mut generation = sync_generation.write();
      *generation += 1;
      *generation
    };

    let handle = handle.clone();

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

      apply_outbound(handle, changes).await;
    });
  });

  store
}
