use opentelemetry::global;
use opentelemetry::metrics::{Counter, Histogram, UpDownCounter};

pub const METER_NAME: &str = "lightnotes-api";

const DURATION_BUCKETS: [f64; 14] = [
  0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0,
];

#[derive(Clone)]
pub struct AppMetrics {
  pub http_request_duration: Histogram<f64>,
  pub http_active_requests: UpDownCounter<i64>,
  pub mongo_command_duration: Histogram<f64>,
  pub sse_active_streams: UpDownCounter<i64>,
  pub changes_processed: Counter<u64>,
  pub auth_attempts: Counter<u64>,
}

impl AppMetrics {
  pub fn new() -> Self {
    let meter = global::meter(METER_NAME);

    Self {
      http_request_duration: meter
        .f64_histogram("http.server.request.duration")
        .with_unit("s")
        .with_description("Duration of inbound HTTP requests")
        .with_boundaries(DURATION_BUCKETS.to_vec())
        .build(),
      http_active_requests: meter
        .i64_up_down_counter("http.server.active_requests")
        .with_unit("{request}")
        .with_description("Number of in-flight inbound HTTP requests")
        .build(),
      mongo_command_duration: meter
        .f64_histogram("db.client.operation.duration")
        .with_unit("s")
        .with_description("Duration of MongoDB commands")
        .with_boundaries(DURATION_BUCKETS.to_vec())
        .build(),
      sse_active_streams: meter
        .i64_up_down_counter("lightnotes.sse.active_streams")
        .with_unit("{stream}")
        .with_description("Number of open change-stream SSE connections")
        .build(),
      changes_processed: meter
        .u64_counter("lightnotes.changes.processed")
        .with_unit("{change}")
        .with_description("Changes pushed to the sync API by outcome")
        .build(),
      auth_attempts: meter
        .u64_counter("lightnotes.auth.attempts")
        .with_unit("{attempt}")
        .with_description("Authentication attempts by method and outcome")
        .build(),
    }
  }
}

impl Default for AppMetrics {
  fn default() -> Self {
    Self::new()
  }
}
