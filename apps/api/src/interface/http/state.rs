use std::sync::Arc;

use crate::application::commands::google_sign_in::GoogleSignInHandler;
use crate::application::commands::native_auth::NativeAuthHandler;
use crate::application::commands::push_changes::PushChangesHandler;
use crate::application::commands::refresh_session::RefreshSessionHandler;
use crate::application::commands::sign_out::SignOutHandler;
use crate::application::queries::current_user::CurrentUserHandler;
use crate::application::queries::pull_changes::PullChangesHandler;
use crate::application::queries::stream_changes::StreamChangesHandler;
use crate::domain::ports::TokenIssuer;

#[derive(Clone)]
pub struct AppState {
  pub push_handler: Arc<PushChangesHandler>,
  pub pull_handler: Arc<PullChangesHandler>,
  pub stream_handler: Arc<StreamChangesHandler>,
  pub google_sign_in_handler: Arc<GoogleSignInHandler>,
  pub native_auth_handler: Arc<NativeAuthHandler>,
  pub refresh_session_handler: Arc<RefreshSessionHandler>,
  pub sign_out_handler: Arc<SignOutHandler>,
  pub current_user_handler: Arc<CurrentUserHandler>,
  pub token_issuer: Arc<dyn TokenIssuer>,
  pub google_client_id: String,
}
