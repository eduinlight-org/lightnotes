use std::sync::Arc;
use std::time::Instant;

use axum::body::Body;
use axum::extract::{MatchedPath, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use futures_util::StreamExt;
use opentelemetry::global;
use opentelemetry::KeyValue;
use opentelemetry_http::HeaderExtractor;
use opentelemetry_semantic_conventions::attribute::{HTTP_REQUEST_METHOD, HTTP_RESPONSE_STATUS_CODE, HTTP_ROUTE};
use sync_dto::PushChangesResponse;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::infrastructure::telemetry::AppMetrics;

const UNMATCHED_ROUTE: &str = "unmatched";

fn matched_route<B>(request: &axum::http::Request<B>) -> String {
  request
    .extensions()
    .get::<MatchedPath>()
    .map(|path| path.as_str().to_string())
    .unwrap_or_else(|| UNMATCHED_ROUTE.to_string())
}

pub fn make_span<B>(request: &axum::http::Request<B>) -> Span {
  let method = request.method().as_str();
  let route = matched_route(request);

  let span = tracing::info_span!(
    "http_request",
    otel.name = format!("{method} {route}"),
    otel.kind = "server",
    "http.request.method" = method,
    "http.route" = route,
    "url.path" = request.uri().path(),
    "http.response.status_code" = tracing::field::Empty,
  );

  let parent = global::get_text_map_propagator(|propagator| propagator.extract(&HeaderExtractor(request.headers())));
  let _ = span.set_parent(parent);

  span
}

struct InFlightGuard {
  metrics: Arc<AppMetrics>,
  attributes: [KeyValue; 1],
}

impl InFlightGuard {
  fn enter(metrics: Arc<AppMetrics>, method: String) -> Self {
    let attributes = [KeyValue::new(HTTP_REQUEST_METHOD, method)];
    metrics.http_active_requests.add(1, &attributes);
    Self { metrics, attributes }
  }
}

impl Drop for InFlightGuard {
  fn drop(&mut self) {
    self.metrics.http_active_requests.add(-1, &self.attributes);
  }
}

pub async fn record_requests(State(metrics): State<Arc<AppMetrics>>, request: Request, next: Next) -> Response {
  let method = request.method().as_str().to_string();
  let route = matched_route(&request);

  let in_flight = InFlightGuard::enter(metrics.clone(), method.clone());

  let started = Instant::now();
  let response = next.run(request).await;
  let elapsed = started.elapsed().as_secs_f64();

  drop(in_flight);

  let status = response.status().as_u16();
  Span::current().record("http.response.status_code", status);

  metrics.http_request_duration.record(
    elapsed,
    &[
      KeyValue::new(HTTP_REQUEST_METHOD, method),
      KeyValue::new(HTTP_ROUTE, route),
      KeyValue::new(HTTP_RESPONSE_STATUS_CODE, i64::from(status)),
    ],
  );

  response
}

#[derive(Clone)]
pub struct AuthRoute {
  pub metrics: Arc<AppMetrics>,
  pub method: &'static str,
}

fn auth_outcome(status: StatusCode) -> &'static str {
  match status {
    StatusCode::ACCEPTED => "pending",
    status if status.is_success() => "success",
    status if status.is_server_error() => "error",
    _ => "failure",
  }
}

pub async fn record_auth_attempt(State(route): State<AuthRoute>, request: Request, next: Next) -> Response {
  let response = next.run(request).await;

  route.metrics.auth_attempts.add(
    1,
    &[
      KeyValue::new("auth.method", route.method),
      KeyValue::new("outcome", auth_outcome(response.status())),
    ],
  );

  response
}

pub async fn record_pushed_changes(State(metrics): State<Arc<AppMetrics>>, request: Request, next: Next) -> Response {
  let response = next.run(request).await;

  if !response.status().is_success() {
    return response;
  }

  let (parts, body) = response.into_parts();

  let bytes = match axum::body::to_bytes(body, usize::MAX).await {
    Ok(bytes) => bytes,
    Err(error) => {
      tracing::error!(%error, "could not read push response while recording metrics");
      return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
  };

  match serde_json::from_slice::<PushChangesResponse>(&bytes) {
    Ok(pushed) => {
      metrics
        .changes_processed
        .add(pushed.accepted.len() as u64, &[KeyValue::new("outcome", "accepted")]);
      metrics
        .changes_processed
        .add(pushed.rejected.len() as u64, &[KeyValue::new("outcome", "rejected")]);
    }
    Err(error) => tracing::warn!(%error, "unrecognised push response, change counts not recorded"),
  }

  Response::from_parts(parts, Body::from(bytes))
}

struct StreamGuard {
  metrics: Arc<AppMetrics>,
}

impl StreamGuard {
  fn open(metrics: Arc<AppMetrics>) -> Self {
    metrics.sse_active_streams.add(1, &[]);
    Self { metrics }
  }
}

impl Drop for StreamGuard {
  fn drop(&mut self) {
    self.metrics.sse_active_streams.add(-1, &[]);
  }
}

pub async fn track_open_stream(State(metrics): State<Arc<AppMetrics>>, request: Request, next: Next) -> Response {
  let response = next.run(request).await;

  if !response.status().is_success() {
    return response;
  }

  let guard = StreamGuard::open(metrics);

  response.map(|body| {
    Body::from_stream(body.into_data_stream().map(move |chunk| {
      let _open = &guard;
      chunk
    }))
  })
}
