use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Html;
use axum::Json;
use opentelemetry::KeyValue;
use serde::Deserialize;
use sync_dto::{
  AuthConfigResponse, AuthSessionResponse, AuthTokensDto, CurrentUserResponse, GoogleSignInRequest, NativeAuthPollRequest,
  NativeAuthStartResponse, RefreshRequest, RefreshResponse, SignOutRequest, UserDto,
};

use crate::application::commands::google_sign_in::{GoogleSignInCommand, IssuedSession};
use crate::application::commands::native_auth::PollOutcome;
use crate::application::commands::refresh_session::RefreshSessionCommand;
use crate::application::commands::sign_out::SignOutCommand;
use crate::application::queries::current_user::CurrentUserQuery;
use crate::domain::user::{AuthError, User};

use super::auth_user::AuthUser;
use super::state::AppState;

fn now_ms() -> i64 {
  std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .expect("system clock before unix epoch")
    .as_millis() as i64
}

fn to_user_dto(user: User) -> UserDto {
  UserDto {
    id: user.id,
    email: user.email,
    name: user.name,
    picture: user.picture,
  }
}

fn to_tokens_dto(session: &IssuedSession) -> AuthTokensDto {
  AuthTokensDto {
    access_token: session.access_token.clone(),
    refresh_token: session.refresh_token.clone(),
    expires_in_secs: session.expires_in_secs,
  }
}

fn record_auth(state: &AppState, method: &'static str, outcome: &'static str) {
  state
    .metrics
    .auth_attempts
    .add(1, &[KeyValue::new("auth.method", method), KeyValue::new("outcome", outcome)]);
}

fn status_for(error: &AuthError) -> StatusCode {
  match error {
    AuthError::Backend(_) => StatusCode::INTERNAL_SERVER_ERROR,
    _ => StatusCode::UNAUTHORIZED,
  }
}

pub async fn config(State(state): State<AppState>) -> Json<AuthConfigResponse> {
  Json(AuthConfigResponse {
    google_client_id: state.google_client_id.clone(),
  })
}

pub async fn google_sign_in(
  State(state): State<AppState>,
  Json(request): Json<GoogleSignInRequest>,
) -> Result<Json<AuthSessionResponse>, StatusCode> {
  let device_id = Some(request.device_id).filter(|value| !value.is_empty());

  let session = state
    .google_sign_in_handler
    .handle(
      GoogleSignInCommand {
        id_token: request.id_token,
        device_id,
      },
      now_ms(),
    )
    .await
    .map_err(|err| {
      tracing::warn!("google sign in failed: {err}");
      record_auth(&state, "google", "failure");
      status_for(&err)
    })?;

  record_auth(&state, "google", "success");

  let tokens = to_tokens_dto(&session);

  Ok(Json(AuthSessionResponse {
    user: to_user_dto(session.user),
    tokens,
  }))
}

pub async fn refresh(State(state): State<AppState>, Json(request): Json<RefreshRequest>) -> Result<Json<RefreshResponse>, StatusCode> {
  let session = state
    .refresh_session_handler
    .handle(
      RefreshSessionCommand {
        refresh_token: request.refresh_token,
      },
      now_ms(),
    )
    .await
    .map_err(|err| {
      tracing::warn!("session refresh failed: {err}");
      record_auth(&state, "refresh", "failure");
      status_for(&err)
    })?;

  record_auth(&state, "refresh", "success");

  Ok(Json(RefreshResponse {
    tokens: to_tokens_dto(&session),
  }))
}

pub async fn logout(State(state): State<AppState>, Json(request): Json<SignOutRequest>) -> Result<StatusCode, StatusCode> {
  state
    .sign_out_handler
    .handle(
      SignOutCommand {
        refresh_token: request.refresh_token,
      },
      now_ms(),
    )
    .await
    .map_err(|err| {
      tracing::warn!("sign out failed: {err}");
      record_auth(&state, "logout", "failure");
      status_for(&err)
    })?;

  record_auth(&state, "logout", "success");

  Ok(StatusCode::NO_CONTENT)
}

pub async fn native_start(State(state): State<AppState>) -> Result<Json<NativeAuthStartResponse>, StatusCode> {
  let started = state.native_auth_handler.start(now_ms()).await.map_err(|err| {
    tracing::warn!("native auth start failed: {err}");
    record_auth(&state, "native_start", "failure");
    status_for(&err)
  })?;

  record_auth(&state, "native_start", "success");

  Ok(Json(NativeAuthStartResponse {
    ticket: started.ticket,
    authorize_url: started.authorize_url,
  }))
}

#[derive(Debug, Deserialize)]
pub struct NativeCallbackParams {
  #[serde(default)]
  code: String,
  #[serde(default)]
  state: String,
}

fn callback_page(message: &str) -> Html<String> {
  Html(format!(
    "<!doctype html><html><head><meta charset=\"utf-8\"><title>LightNotes</title></head>\
     <body style=\"font-family:system-ui;display:flex;align-items:center;justify-content:center;height:100vh;margin:0\">\
     <p>{message}</p></body></html>"
  ))
}

pub async fn native_callback(State(state): State<AppState>, Query(params): Query<NativeCallbackParams>) -> Html<String> {
  if params.code.is_empty() || params.state.is_empty() {
    record_auth(&state, "native_callback", "cancelled");
    return callback_page("Sign-in was cancelled. You can close this window.");
  }

  match state.native_auth_handler.complete(&params.code, &params.state, now_ms()).await {
    Ok(()) => {
      record_auth(&state, "native_callback", "success");
      callback_page("Signed in. You can close this window and return to LightNotes.")
    }
    Err(err) => {
      tracing::warn!("native auth callback failed: {err}");
      record_auth(&state, "native_callback", "failure");
      callback_page("Sign-in failed. You can close this window and try again.")
    }
  }
}

pub async fn native_poll(
  State(state): State<AppState>,
  Json(request): Json<NativeAuthPollRequest>,
) -> Result<(StatusCode, Json<Option<AuthSessionResponse>>), StatusCode> {
  let outcome = state.native_auth_handler.poll(&request.ticket).await.map_err(|err| {
    tracing::warn!("native auth poll failed: {err}");
    record_auth(&state, "native_poll", "failure");
    status_for(&err)
  })?;

  match outcome {
    PollOutcome::Pending => Ok((StatusCode::ACCEPTED, Json(None))),
    PollOutcome::Failed => {
      record_auth(&state, "native_poll", "failure");
      Err(StatusCode::UNAUTHORIZED)
    }
    PollOutcome::Complete(session) => {
      record_auth(&state, "native_poll", "success");
      Ok((
        StatusCode::OK,
        Json(Some(AuthSessionResponse {
          user: to_user_dto(session.user),
          tokens: AuthTokensDto {
            access_token: session.access_token,
            refresh_token: session.refresh_token,
            expires_in_secs: session.expires_in_secs,
          },
        })),
      ))
    }
  }
}

pub async fn me(State(state): State<AppState>, user: AuthUser) -> Result<Json<CurrentUserResponse>, StatusCode> {
  let found = state
    .current_user_handler
    .handle(CurrentUserQuery { user_id: user.user_id })
    .await
    .map_err(|err| status_for(&err))?;

  Ok(Json(CurrentUserResponse {
    user: to_user_dto(found),
  }))
}
