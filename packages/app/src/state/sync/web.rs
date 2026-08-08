use std::sync::Arc;

use api_sdk::{ApiClient, SseChangeEvent};
use dioxus::prelude::*;
use futures_util::StreamExt;

use super::dto::{api_base_url, diff_folders, diff_notes, diff_tags, merge_server_changes};
use crate::state::auth::{use_auth, AuthState, AuthStatus};
use crate::state::boot::use_boot;
use crate::state::notes::{Folder, Note, NotesStore, SyncStatus, Tag};
use crate::state::preferences::use_persisted_preferences;

const BASE_BACKOFF_MS: u32 = 1000;
const MAX_BACKOFF_MS: u32 = 30_000;
const OUTBOUND_DEBOUNCE_MS: u32 = 600;
const DEVICE_KEY: &str = "lightnotes:device:v1";

fn use_device_id() -> Signal<String> {
  let mut device = use_signal(String::new);

  use_hook(move || {
    spawn(async move {
      let mut eval = document::eval(&format!(
        "let existing = localStorage.getItem('{DEVICE_KEY}');
         if (!existing) {{ existing = crypto.randomUUID(); localStorage.setItem('{DEVICE_KEY}', existing); }}
         dioxus.send(existing);"
      ));

      if let Ok(value) = eval.recv::<String>().await {
        device.set(value);
      } else {
        device.set(uuid::Uuid::new_v4().to_string());
      }
    });
  });

  device
}

fn persist_tokens(api: &ApiClient, mut auth: AuthState) {
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
  let api = use_hook(|| Arc::new(ApiClient::new(api_base_url())));
  let device = use_device_id();
  let mut loaded = boot.store_ready;
  let session_ready = boot.session_ready;
  let mut last_synced_notes = use_signal(Vec::<Note>::new);
  let mut last_synced_folders = use_signal(Vec::<Folder>::new);
  let mut last_synced_tags = use_signal(Vec::<Tag>::new);
  let mut sync_generation = use_signal(|| 0u64);
  let mut stream_task = use_signal(|| None::<dioxus::core::Task>);

  let reset_api = api.clone();
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
    loaded.set(false);

    store.clear_synced_entities();
    last_synced_notes.set(Vec::new());
    last_synced_folders.set(Vec::new());
    last_synced_tags.set(Vec::new());

    reset_api.set_tokens(tokens);

    let Some(user_id) = user_id.filter(|_| signed_in) else {
      loaded.set(true);
      return;
    };

    store.set_user(user_id.clone());

    let stream_api = reset_api.clone();
    let task = spawn(async move {
      let mut since = 0i64;
      let mut backoff_ms = BASE_BACKOFF_MS;

      loop {
        if store.sync() == SyncStatus::Offline {
          gloo_timers::future::TimeoutFuture::new(BASE_BACKOFF_MS).await;
          continue;
        }

        let this_device = device.peek().clone();
        let mut stream = stream_api.subscribe_changes(since);

        while let Some(event) = stream.next().await {
          backoff_ms = BASE_BACKOFF_MS;

          match event {
            SseChangeEvent::Change(change) => {
              since = since.max(change.seq);

              if change.device_id != this_device {
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

              if !*loaded.peek() {
                let baseline = store.snapshot();
                last_synced_notes.set(baseline.notes);
                last_synced_folders.set(baseline.folders);
                last_synced_tags.set(baseline.tags);
                loaded.set(true);
              }
            }
          }
        }

        drop(stream);
        persist_tokens(&stream_api, auth);

        gloo_timers::future::TimeoutFuture::new(backoff_ms).await;
        backoff_ms = (backoff_ms * 2).min(MAX_BACKOFF_MS);
      }
    });

    stream_task.set(Some(task));
  });

  let outbound_api = api.clone();
  use_effect(move || {
    let is_loaded = loaded();
    let is_offline = store.sync() == SyncStatus::Offline;
    let is_signed_in = auth.is_signed_in();
    let _ = store.snapshot();

    if !is_loaded || is_offline || !is_signed_in {
      return;
    }

    let generation = {
      let mut generation = sync_generation.write();
      *generation += 1;
      *generation
    };

    let api = outbound_api.clone();

    spawn(async move {
      gloo_timers::future::TimeoutFuture::new(OUTBOUND_DEBOUNCE_MS).await;

      if *sync_generation.peek() != generation {
        return;
      }

      let this_device = device.peek().clone();
      let current = store.snapshot();
      let previous_notes = last_synced_notes.peek().clone();
      let previous_folders = last_synced_folders.peek().clone();
      let previous_tags = last_synced_tags.peek().clone();

      let mut changes = diff_notes(&previous_notes, &current.notes, &this_device);
      changes.extend(diff_folders(&previous_folders, &current.folders, &this_device));
      changes.extend(diff_tags(&previous_tags, &current.tags, &this_device));

      last_synced_notes.set(current.notes);
      last_synced_folders.set(current.folders);
      last_synced_tags.set(current.tags);

      for change in changes {
        let _ = api.push_changes(vec![change]).await;
      }

      persist_tokens(&api, auth);
    });
  });

  store
}
