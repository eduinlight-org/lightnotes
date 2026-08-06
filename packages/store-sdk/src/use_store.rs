use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use api_sdk::{ApiClient, SseChangeEvent};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use futures_util::Stream;
use tokio::sync::OnceCell;

use crate::config::StoreConfig;
use crate::local_store::{ApplyOutcome, LocalSnapshot, LocalStore};
use crate::processor;
use sync_dto::QueuedChange;

#[derive(Clone)]
pub struct StoreHandle {
  cell: Arc<OnceCell<Option<LocalStore>>>,
  db_path: PathBuf,
  api: Arc<ApiClient>,
}

impl StoreHandle {
  async fn store(&self) -> Option<&LocalStore> {
    self.cell.get_or_init(|| async { LocalStore::try_connect(&self.db_path).await }).await.as_ref()
  }

  pub async fn load_snapshot(&self, user_id: &str) -> LocalSnapshot {
    let Some(store) = self.store().await else {
      return LocalSnapshot::default();
    };

    store.load_snapshot(user_id).await
  }

  pub async fn apply(&self, user_id: &str, change: &QueuedChange) -> ApplyOutcome {
    let Some(store) = self.store().await else {
      return ApplyOutcome::Skipped;
    };

    store.apply(user_id, change).await
  }

  pub async fn enqueue_outbound(&self, user_id: &str, change: &QueuedChange) {
    let Some(store) = self.store().await else {
      return;
    };

    store.enqueue_outbound(user_id, change).await
  }

  pub async fn device_id(&self) -> String {
    let Some(store) = self.store().await else {
      return uuid::Uuid::new_v4().to_string();
    };

    store.device_id().await
  }

  pub async fn peek_front_outbound(&self, user_id: &str) -> Option<(i64, QueuedChange)> {
    self.store().await?.peek_front_outbound(user_id).await
  }

  pub async fn dequeue_front_outbound(&self, id: i64) {
    let Some(store) = self.store().await else {
      return;
    };

    store.dequeue_front_outbound(id).await
  }

  pub async fn cursor(&self, user_id: &str) -> i64 {
    let Some(store) = self.store().await else {
      return 0;
    };

    store.cursor(user_id).await
  }

  pub async fn set_cursor(&self, user_id: &str, cursor: i64) {
    let Some(store) = self.store().await else {
      return;
    };

    store.set_cursor(user_id, cursor).await
  }

  pub async fn load_session(&self) -> Option<(String, String)> {
    self.store().await?.load_session().await
  }

  pub async fn save_session(&self, user_id: &str, session_json: &str, updated_at_ms: i64) {
    let Some(store) = self.store().await else {
      return;
    };

    store.save_session(user_id, session_json, updated_at_ms).await
  }

  pub async fn clear_session(&self) {
    let Some(store) = self.store().await else {
      return;
    };

    store.clear_session().await
  }

  pub async fn clear_user_data(&self, user_id: &str) {
    let Some(store) = self.store().await else {
      return;
    };

    store.clear_user_data(user_id).await
  }

  pub fn api(&self) -> Arc<ApiClient> {
    self.api.clone()
  }

  pub async fn push_changes(&self, changes: Vec<QueuedChange>) -> Result<(), ()> {
    self.api.push_changes(changes).await.map(|_| ()).map_err(|_| ())
  }

  pub fn subscribe_changes(&self, since: i64) -> Pin<Box<dyn Stream<Item = SseChangeEvent> + Send + '_>> {
    self.api.subscribe_changes(since)
  }
}

pub fn use_synced_store(config: StoreConfig, offline: Signal<bool>, active_user: Signal<Option<String>>) -> StoreHandle {
  use_hook(|| {
    info!("local store path: {}", config.db_path.display());

    let api = Arc::new(ApiClient::new(config.api_base_url.clone()));
    let handle = StoreHandle { cell: Arc::new(OnceCell::new()), db_path: config.db_path.clone(), api };

    let processor_handle = handle.clone();
    spawn(async move {
      processor::run(processor_handle, offline, active_user).await;
    });

    handle
  })
}
