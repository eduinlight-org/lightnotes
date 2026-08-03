use std::pin::Pin;
use std::sync::Arc;

use futures_util::stream::{self, Stream, StreamExt};

use crate::domain::ports::{ChangeNotifier, ChangeRepository, RepositoryError, StoredChange};

pub struct StreamChangesQuery {
  pub user_id: String,
  pub since: i64,
}

pub enum StreamItem {
  Change(Box<StoredChange>),
  CaughtUp { cursor: i64 },
}

pub struct StreamChangesHandler {
  pub change_repo: Arc<dyn ChangeRepository>,
  pub notifier: Arc<dyn ChangeNotifier>,
}

impl StreamChangesHandler {
  pub async fn handle(&self, query: StreamChangesQuery) -> Result<Pin<Box<dyn Stream<Item = StreamItem> + Send>>, RepositoryError> {
    let live = self.notifier.subscribe(&query.user_id);
    let catch_up = self.change_repo.list_since(&query.user_id, query.since).await?;
    let last_seq = catch_up.iter().map(|change| change.seq).max().unwrap_or(query.since);

    let catch_up_stream = stream::iter(catch_up.into_iter().map(|change| StreamItem::Change(Box::new(change))));
    let caught_up_marker = stream::once(async move { StreamItem::CaughtUp { cursor: last_seq } });
    let live_stream = live
      .filter(move |change| {
        let seq = change.seq;
        async move { seq > last_seq }
      })
      .map(|change| StreamItem::Change(Box::new(change)));

    Ok(Box::pin(catch_up_stream.chain(caught_up_marker).chain(live_stream)))
  }
}
