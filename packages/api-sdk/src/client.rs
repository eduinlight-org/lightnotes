use std::fmt;
use std::pin::Pin;

use async_stream::stream;
use futures_util::{Stream, StreamExt};
use sync_dto::{PullChangesResponse, PushChangesRequest, PushChangesResponse, QueuedChange, ServerChange};

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
}

#[derive(Debug)]
pub enum ApiSdkError {
  Transport(reqwest::Error),
  Status(reqwest::StatusCode),
}

impl fmt::Display for ApiSdkError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ApiSdkError::Transport(err) => write!(f, "transport error: {err}"),
      ApiSdkError::Status(status) => write!(f, "unexpected status: {status}"),
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
    Self { base_url: base_url.into(), http: reqwest::Client::new() }
  }

  pub async fn push_changes(&self, changes: Vec<QueuedChange>) -> Result<PushChangesResponse, ApiSdkError> {
    let response = self
      .http
      .post(format!("{}/api/v1/changes", self.base_url))
      .json(&PushChangesRequest { changes })
      .send()
      .await?;

    if !response.status().is_success() {
      return Err(ApiSdkError::Status(response.status()));
    }

    Ok(response.json::<PushChangesResponse>().await?)
  }

  pub async fn pull_changes(&self, since: i64) -> Result<PullChangesResponse, ApiSdkError> {
    let response = self
      .http
      .get(format!("{}/api/v1/changes", self.base_url))
      .query(&[("since", since)])
      .send()
      .await?;

    if !response.status().is_success() {
      return Err(ApiSdkError::Status(response.status()));
    }

    Ok(response.json::<PullChangesResponse>().await?)
  }

  fn subscribe_changes_inner(&self, since: i64) -> impl Stream<Item = SseChangeEvent> {
    let url = format!("{}/api/v1/changes/stream", self.base_url);
    let http = self.http.clone();

    stream! {
      let Ok(response) = http.get(&url).query(&[("since", since)]).send().await else {
        return;
      };

      if !response.status().is_success() {
        return;
      }

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
  pub fn subscribe_changes(&self, since: i64) -> Pin<Box<dyn Stream<Item = SseChangeEvent> + Send>> {
    Box::pin(self.subscribe_changes_inner(since))
  }

  #[cfg(target_arch = "wasm32")]
  pub fn subscribe_changes(&self, since: i64) -> Pin<Box<dyn Stream<Item = SseChangeEvent>>> {
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
