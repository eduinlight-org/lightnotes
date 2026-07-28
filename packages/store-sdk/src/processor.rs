use std::time::Duration;

use crate::use_store::StoreHandle;

const BASE_BACKOFF_MS: u64 = 1000;
const MAX_BACKOFF_MS: u64 = 30_000;

pub async fn run(store: StoreHandle) {
  let mut backoff_ms = BASE_BACKOFF_MS;

  loop {
    let Some((row_id, change)) = store.peek_front_outbound().await else {
      tokio::time::sleep(Duration::from_millis(BASE_BACKOFF_MS)).await;
      continue;
    };

    match store.push_changes(vec![change]).await {
      Ok(()) => {
        store.dequeue_front_outbound(row_id).await;
        backoff_ms = BASE_BACKOFF_MS;
      }
      Err(()) => {
        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
        backoff_ms = (backoff_ms * 2).min(MAX_BACKOFF_MS);
      }
    }
  }
}
