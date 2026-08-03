use std::pin::Pin;

use futures_util::{future, Stream, StreamExt};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

use crate::domain::ports::{ChangeNotifier, StoredChange};

pub struct BroadcastChangeNotifier {
  sender: broadcast::Sender<StoredChange>,
}

impl BroadcastChangeNotifier {
  pub fn new() -> Self {
    let (sender, _) = broadcast::channel(1024);
    Self { sender }
  }
}

impl ChangeNotifier for BroadcastChangeNotifier {
  fn notify(&self, change: StoredChange) {
    let _ = self.sender.send(change);
  }

  fn subscribe(&self, user_id: &str) -> Pin<Box<dyn Stream<Item = StoredChange> + Send>> {
    let receiver = self.sender.subscribe();
    let user_id = user_id.to_string();

    let stream = BroadcastStream::new(receiver)
      .take_while(|item| future::ready(item.is_ok()))
      .filter_map(|item| async move { item.ok() })
      .filter(move |change| future::ready(change.user_id == user_id));

    Box::pin(stream)
  }
}
