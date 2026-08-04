use std::fmt;
use std::pin::Pin;
use std::sync::Mutex;

use async_stream::stream;
use futures_util::{Stream, StreamExt};
use sync_dto::{
  AuthConfigResponse, AuthSessionResponse, AuthTokensDto, CurrentUserResponse, GoogleSignInRequest, NativeAuthPollRequest,
  NativeAuthStartResponse, PullChangesResponse, PushChangesRequest, PushChangesResponse, QueuedChange, RefreshRequest, RefreshResponse,
  ServerChange, SignOutRequest, UserDto,
};

#[derive(Debug, Clone)]
pub enum SseChangeEvent {
  Change(Box<ServerChange>),
  CaughtUp { cursor: i64 },
}

fn parse_sse_event(block: &str) -> Option<SseChangeEvent> {
  let mut event_type = "message".to_string();
  let mut data_lines = Vec::new();

  for line in block.lines() {
    if let Some(value) = line.strip_prefix("event:") {
      event_type = value.trim().to_string();
    } else if let Some(value) = line.strip_prefix("data:") {
      data_lines.push(value.trim_start());
    }
  }

  if data_lines.is_empty() {
    return None;
  }

  let data = data_lines.join("\n");

  match event_type.as_str() {
    "change" => serde_json::from_str::<ServerChange>(&data).ok().map(|change| SseChangeEvent::Change(Box::new(change))),
    "caught-up" => data.parse::<i64>().ok().map(|cursor| SseChangeEvent::CaughtUp { cursor }),
    _ => None,
  }
}

pub struct ApiClient {
  base_url: String,
  http: reqwest::Client,
  tokens: Mutex<Option<AuthTokensDto>>,
  rotated: Mutex<Option<AuthTokensDto>>,
  refresh_lock: futures_util::lock::Mutex<()>,
}

#[derive(Debug)]
pub enum ApiSdkError {
  Transport(reqwest::Error),
  Status(reqwest::StatusCode),
  Unauthorized,
}

impl fmt::Display for ApiSdkError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ApiSdkError::Transport(err) => write!(f, "transport error: {err}"),
      ApiSdkError::Status(status) => write!(f, "unexpected status: {status}"),
      ApiSdkError::Unauthorized => write!(f, "unauthorized"),
    }
  }
}

impl std::error::Error for ApiSdkError {}

impl From<reqwest::Error> for ApiSdkError {
  fn from(err: reqwest::Error) -> Self {
    ApiSdkError::Transport(err)
  }
}

impl ApiClient {
  pub fn new(base_url: impl Into<String>) -> Self {
    Self {
      base_url: base_url.into(),
      http: reqwest::Client::new(),
      tokens: Mutex::new(None),
      rotated: Mutex::new(None),
      refresh_lock: futures_util::lock::Mutex::new(()),
    }
  }

  pub fn set_tokens(&self, tokens: Option<AuthTokensDto>) {
    *self.tokens.lock().expect("token slot poisoned") = tokens;
  }

  pub fn access_token(&self) -> Option<String> {
    self
      .tokens
      .lock()
      .expect("token slot poisoned")
      .as_ref()
      .map(|tokens| tokens.access_token.clone())
  }

  fn refresh_token(&self) -> Option<String> {
    self
      .tokens
      .lock()
      .expect("token slot poisoned")
      .as_ref()
      .map(|tokens| tokens.refresh_token.clone())
  }

  pub fn take_rotated_tokens(&self) -> Option<AuthTokensDto> {
    self.rotated.lock().expect("rotation slot poisoned").take()
  }

  fn store_rotated(&self, tokens: AuthTokensDto) {
    *self.tokens.lock().expect("token slot poisoned") = Some(tokens.clone());
    *self.rotated.lock().expect("rotation slot poisoned") = Some(tokens);
  }

  async fn refresh_once(&self, used_access_token: Option<String>) -> Result<String, ApiSdkError> {
    let _guard = self.refresh_lock.lock().await;

    let current = self.access_token();
    if current.is_some() && current != used_access_token {
      return current.ok_or(ApiSdkError::Unauthorized);
    }

    let refresh_token = self.refresh_token().ok_or(ApiSdkError::Unauthorized)?;

    let response = self
      .http
      .post(format!("{}/api/v1/auth/refresh", self.base_url))
      .json(&RefreshRequest { refresh_token })
      .send()
      .await?;

    if response.status() == reqwest::StatusCode::UNAUTHORIZED {
      self.set_tokens(None);
      return Err(ApiSdkError::Unauthorized);
    }

    if !response.status().is_success() {
      return Err(ApiSdkError::Status(response.status()));
    }

    let refreshed = response.json::<RefreshResponse>().await?;
    let access_token = refreshed.tokens.access_token.clone();
    self.store_rotated(refreshed.tokens);

    Ok(access_token)
  }

  async fn send_authorized<F>(&self, build: F) -> Result<reqwest::Response, ApiSdkError>
  where
    F: Fn(&reqwest::Client, Option<&str>) -> reqwest::RequestBuilder,
  {
    let access_token = self.access_token();
    let response = build(&self.http, access_token.as_deref()).send().await?;

    if response.status() != reqwest::StatusCode::UNAUTHORIZED {
      if !response.status().is_success() {
        return Err(ApiSdkError::Status(response.status()));
      }

      return Ok(response);
    }

    let refreshed = self.refresh_once(access_token).await?;
    let retried = build(&self.http, Some(&refreshed)).send().await?;

    if retried.status() == reqwest::StatusCode::UNAUTHORIZED {
      return Err(ApiSdkError::Unauthorized);
    }

    if !retried.status().is_success() {
      return Err(ApiSdkError::Status(retried.status()));
    }

    Ok(retried)
  }

  pub async fn auth_config(&self) -> Result<AuthConfigResponse, ApiSdkError> {
    let response = self.http.get(format!("{}/api/v1/auth/config", self.base_url)).send().await?;

    if !response.status().is_success() {
      return Err(ApiSdkError::Status(response.status()));
    }

    Ok(response.json::<AuthConfigResponse>().await?)
  }

  pub async fn google_sign_in(&self, id_token: String, device_id: String) -> Result<(UserDto, AuthTokensDto), ApiSdkError> {
    let response = self
      .http
      .post(format!("{}/api/v1/auth/google", self.base_url))
      .json(&GoogleSignInRequest { id_token, device_id })
      .send()
      .await?;

    if response.status() == reqwest::StatusCode::UNAUTHORIZED {
      return Err(ApiSdkError::Unauthorized);
    }

    if !response.status().is_success() {
      return Err(ApiSdkError::Status(response.status()));
    }

    let session = response.json::<AuthSessionResponse>().await?;
    self.set_tokens(Some(session.tokens.clone()));

    Ok((session.user, session.tokens))
  }

  pub async fn native_auth_start(&self) -> Result<NativeAuthStartResponse, ApiSdkError> {
    let response = self
      .http
      .post(format!("{}/api/v1/auth/native/start", self.base_url))
      .send()
      .await?;

    if !response.status().is_success() {
      return Err(ApiSdkError::Status(response.status()));
    }

    Ok(response.json::<NativeAuthStartResponse>().await?)
  }

  pub async fn native_auth_poll(&self, ticket: &str) -> Result<Option<(UserDto, AuthTokensDto)>, ApiSdkError> {
    let response = self
      .http
      .post(format!("{}/api/v1/auth/native/poll", self.base_url))
      .json(&NativeAuthPollRequest {
        ticket: ticket.to_string(),
      })
      .send()
      .await?;

    if response.status() == reqwest::StatusCode::ACCEPTED {
      return Ok(None);
    }

    if response.status() == reqwest::StatusCode::UNAUTHORIZED {
      return Err(ApiSdkError::Unauthorized);
    }

    if !response.status().is_success() {
      return Err(ApiSdkError::Status(response.status()));
    }

    let session = response.json::<AuthSessionResponse>().await?;
    self.set_tokens(Some(session.tokens.clone()));

    Ok(Some((session.user, session.tokens)))
  }

  pub async fn logout(&self) -> Result<(), ApiSdkError> {
    let Some(refresh_token) = self.refresh_token() else {
      return Ok(());
    };

    let response = self
      .http
      .post(format!("{}/api/v1/auth/logout", self.base_url))
      .json(&SignOutRequest { refresh_token })
      .send()
      .await;

    self.set_tokens(None);
    let _ = self.take_rotated_tokens();

    response?;

    Ok(())
  }

  pub async fn me(&self) -> Result<UserDto, ApiSdkError> {
    let base_url = self.base_url.clone();

    let response = self
      .send_authorized(move |http, token| {
        let request = http.get(format!("{base_url}/api/v1/auth/me"));
        match token {
          Some(token) => request.bearer_auth(token),
          None => request,
        }
      })
      .await?;

    Ok(response.json::<CurrentUserResponse>().await?.user)
  }

  pub async fn push_changes(&self, changes: Vec<QueuedChange>) -> Result<PushChangesResponse, ApiSdkError> {
    let base_url = self.base_url.clone();
    let body = PushChangesRequest { changes };

    let response = self
      .send_authorized(move |http, token| {
        let request = http.post(format!("{base_url}/api/v1/changes")).json(&body);
        match token {
          Some(token) => request.bearer_auth(token),
          None => request,
        }
      })
      .await?;

    Ok(response.json::<PushChangesResponse>().await?)
  }

  pub async fn pull_changes(&self, since: i64) -> Result<PullChangesResponse, ApiSdkError> {
    let base_url = self.base_url.clone();

    let response = self
      .send_authorized(move |http, token| {
        let request = http.get(format!("{base_url}/api/v1/changes")).query(&[("since", since)]);
        match token {
          Some(token) => request.bearer_auth(token),
          None => request,
        }
      })
      .await?;

    Ok(response.json::<PullChangesResponse>().await?)
  }

  async fn open_stream(&self, since: i64) -> Option<reqwest::Response> {
    let base_url = self.base_url.clone();

    self
      .send_authorized(move |http, token| {
        let request = http.get(format!("{base_url}/api/v1/changes/stream")).query(&[("since", since)]);
        match token {
          Some(token) => request.bearer_auth(token),
          None => request,
        }
      })
      .await
      .ok()
  }

  fn subscribe_changes_inner(&self, since: i64) -> impl Stream<Item = SseChangeEvent> + '_ {
    stream! {
      let Some(response) = self.open_stream(since).await else {
        return;
      };

      let mut byte_stream = response.bytes_stream();
      let mut buffer = String::new();

      while let Some(chunk) = byte_stream.next().await {
        let Ok(chunk) = chunk else {
          return;
        };

        buffer.push_str(&String::from_utf8_lossy(&chunk));

        while let Some(pos) = buffer.find("\n\n") {
          let event_block: String = buffer.drain(..pos + 2).collect();
          if let Some(event) = parse_sse_event(&event_block) {
            yield event;
          }
        }
      }
    }
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn subscribe_changes(&self, since: i64) -> Pin<Box<dyn Stream<Item = SseChangeEvent> + Send + '_>> {
    Box::pin(self.subscribe_changes_inner(since))
  }

  #[cfg(target_arch = "wasm32")]
  pub fn subscribe_changes(&self, since: i64) -> Pin<Box<dyn Stream<Item = SseChangeEvent> + '_>> {
    Box::pin(self.subscribe_changes_inner(since))
  }

  pub async fn health(&self) -> Result<(), ApiSdkError> {
    let response = self.http.get(format!("{}/healthz", self.base_url)).send().await?;

    if !response.status().is_success() {
      return Err(ApiSdkError::Status(response.status()));
    }

    Ok(())
  }
}
