use std::sync::Arc;

use crate::domain::ports::{ChangeRepository, RepositoryError, StoredChange};

pub struct PullChangesQuery {
  pub since: i64,
}

pub struct PullOutcome {
  pub changes: Vec<StoredChange>,
  pub cursor: i64,
}

pub struct PullChangesHandler {
  pub change_repo: Arc<dyn ChangeRepository>,
}

impl PullChangesHandler {
  pub async fn handle(&self, query: PullChangesQuery) -> Result<PullOutcome, RepositoryError> {
    let changes = self.change_repo.list_since(query.since).await?;
    let cursor = changes.iter().map(|change| change.seq).max().unwrap_or(query.since);
    Ok(PullOutcome { changes, cursor })
  }
}
