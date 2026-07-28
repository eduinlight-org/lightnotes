use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use api_sdk::{ApiClient, SseChangeEvent};
use dioxus::prelude::*;
use futures_util::Stream;
use tokio::sync::OnceCell;

use crate::config::StoreConfig;
use crate::local_store::{ApplyOutcome, LocalSnapshot, LocalStore};
use crate::processor;
use sync_dto::QueuedChange;

#[derive(Clone)]
pub struct StoreHandle {
  cell: Arc<OnceCell<LocalStore>>,
  db_path: PathBuf,
  api: Arc<ApiClient>,
}

impl StoreHandle {
  async fn store(&self) -> &LocalStore {
    self.cell.get_or_init(|| async { LocalStore::connect(&self.db_path).await }).await
  }

  pub async fn load_snapshot(&self) -> LocalSnapshot {
    self.store().await.load_snapshot().await
  }

  pub async fn apply(&self, change: &QueuedChange) -> ApplyOutcome {
    self.store().await.apply(change).await
  }

  pub async fn enqueue_outbound(&self, change: &QueuedChange) {
    self.store().await.enqueue_outbound(change).await
  }

  pub async fn device_id(&self) -> String {
    self.store().await.device_id().await
  }

  pub async fn peek_front_outbound(&self) -> Option<(i64, QueuedChange)> {
    self.store().await.peek_front_outbound().await
  }

  pub async fn dequeue_front_outbound(&self, id: i64) {
    self.store().await.dequeue_front_outbound(id).await
  }

  pub async fn cursor(&self) -> i64 {
    self.store().await.cursor().await
  }

  pub async fn set_cursor(&self, cursor: i64) {
    self.store().await.set_cursor(cursor).await
  }

  pub async fn push_changes(&self, changes: Vec<QueuedChange>) -> Result<(), ()> {
    self.api.push_changes(changes).await.map(|_| ()).map_err(|_| ())
  }

  pub fn subscribe_changes(&self, since: i64) -> Pin<Box<dyn Stream<Item = SseChangeEvent> + Send>> {
    self.api.subscribe_changes(since)
  }
}

pub fn use_synced_store(config: StoreConfig) -> StoreHandle {
  use_hook(|| {
    let api = Arc::new(ApiClient::new(config.api_base_url.clone()));
    let handle = StoreHandle { cell: Arc::new(OnceCell::new()), db_path: config.db_path.clone(), api };

    let processor_handle = handle.clone();
    spawn(async move {
      processor::run(processor_handle).await;
    });

    handle
  })
}
