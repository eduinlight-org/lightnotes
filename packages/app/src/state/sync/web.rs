use std::sync::Arc;

use api_sdk::{ApiClient, SseChangeEvent};
use dioxus::prelude::*;
use futures_util::StreamExt;

use super::dto::{api_base_url, diff_folders, diff_notes, diff_tags, merge_server_changes};
use crate::state::notes::{Folder, Note, NotesStore, SyncStatus, Tag};
use crate::state::preferences::use_persisted_preferences;

const BASE_BACKOFF_MS: u32 = 1000;
const MAX_BACKOFF_MS: u32 = 30_000;
const OUTBOUND_DEBOUNCE_MS: u32 = 600;

fn device_id() -> String {
  uuid::Uuid::new_v4().to_string()
}

pub fn use_synced_notes() -> NotesStore {
  let mut store = use_context_provider(NotesStore::seed);
  use_persisted_preferences(store);

  let api = use_hook(|| Arc::new(ApiClient::new(api_base_url())));
  let device = use_hook(device_id);
  let mut loaded = use_signal(|| false);
  let mut last_synced_notes = use_signal(Vec::<Note>::new);
  let mut last_synced_folders = use_signal(Vec::<Folder>::new);
  let mut last_synced_tags = use_signal(Vec::<Tag>::new);
  let mut sync_generation = use_signal(|| 0u64);

  let stream_api = api.clone();
  let stream_device = device.clone();
  use_hook(move || {
    spawn(async move {
      let mut since = 0i64;
      let mut backoff_ms = BASE_BACKOFF_MS;

      loop {
        if store.sync() == SyncStatus::Offline {
          gloo_timers::future::TimeoutFuture::new(BASE_BACKOFF_MS).await;
          continue;
        }

        let mut stream = stream_api.subscribe_changes(since);

        while let Some(event) = stream.next().await {
          backoff_ms = BASE_BACKOFF_MS;

          match event {
            SseChangeEvent::Change(change) => {
              since = since.max(change.seq);

              if change.device_id != stream_device {
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

        gloo_timers::future::TimeoutFuture::new(backoff_ms).await;
        backoff_ms = (backoff_ms * 2).min(MAX_BACKOFF_MS);
      }
    });
  });

  use_effect(move || {
    let is_loaded = loaded();
    let is_offline = store.sync() == SyncStatus::Offline;
    let _ = store.snapshot();

    if !is_loaded || is_offline {
      return;
    }

    let generation = {
      let mut generation = sync_generation.write();
      *generation += 1;
      *generation
    };

    let device = device.clone();
    let api = api.clone();

    spawn(async move {
      gloo_timers::future::TimeoutFuture::new(OUTBOUND_DEBOUNCE_MS).await;

      if *sync_generation.peek() != generation {
        return;
      }

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

      for change in changes {
        let _ = api.push_changes(vec![change]).await;
      }
    });
  });

  store
}
