use std::sync::Arc;

use crate::application::commands::push_changes::PushChangesHandler;
use crate::application::queries::pull_changes::PullChangesHandler;
use crate::application::queries::stream_changes::StreamChangesHandler;

#[derive(Clone)]
pub struct AppState {
  pub push_handler: Arc<PushChangesHandler>,
  pub pull_handler: Arc<PullChangesHandler>,
  pub stream_handler: Arc<StreamChangesHandler>,
}
