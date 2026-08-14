use opentelemetry::trace::TracerProvider as _;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

use crate::infrastructure::config::Config;

use super::providers::{self, Providers};

const TRACER_NAME: &str = "lightnotes-api";

const EXPORT_NOISE_PREFIXES: [&str; 6] = [
  "opentelemetry",
  "hyper",
  "h2",
  "reqwest",
  "rustls",
  "tower::buffer",
];

pub struct TelemetryGuard {
  _log_writer: Option<WorkerGuard>,
  providers: Providers,
}

impl TelemetryGuard {
  pub fn shutdown(&self) {
    self.providers.shutdown();
  }
}

fn is_export_noise(target: &str) -> bool {
  EXPORT_NOISE_PREFIXES
    .iter()
    .any(|prefix| target.starts_with(prefix))
}

pub fn init(config: &Config) -> TelemetryGuard {
  let env_filter =
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug"));

  let providers = providers::init(config);

  let (fmt_layer, log_writer) = if config.is_production() {
    let log_dir = std::env::var("API_LOG_DIR").unwrap_or_else(|_| "logs".to_string());
    std::fs::create_dir_all(&log_dir).expect("failed to create log directory");

    let file_appender = tracing_appender::rolling::daily(log_dir, "api.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let layer = tracing_subscriber::fmt::layer()
      .with_writer(non_blocking)
      .with_ansi(false)
      .json()
      .boxed();

    (layer, Some(guard))
  } else {
    (tracing_subscriber::fmt::layer().boxed(), None)
  };

  let trace_layer = providers.tracer.as_ref().map(|tracer| {
    tracing_opentelemetry::layer()
      .with_tracer(tracer.tracer(TRACER_NAME))
      .with_filter(filter_fn(|metadata| !is_export_noise(metadata.target())))
  });

  let log_layer = providers.logger.as_ref().map(|logger| {
    OpenTelemetryTracingBridge::new(logger).with_filter(filter_fn(|metadata| !is_export_noise(metadata.target())))
  });

  tracing_subscriber::registry()
    .with(env_filter)
    .with(fmt_layer)
    .with(trace_layer)
    .with(log_layer)
    .init();

  match &config.otlp_endpoint {
    Some(endpoint) => tracing::info!(
      otlp.endpoint = %endpoint,
      otlp.traces = providers.tracer.is_some(),
      otlp.metrics = providers.meter.is_some(),
      otlp.logs = providers.logger.is_some(),
      service.name = %config.service_name,
      "opentelemetry export enabled"
    ),
    None => tracing::info!("opentelemetry export disabled, OTEL_EXPORTER_OTLP_ENDPOINT is unset"),
  }

  TelemetryGuard {
    _log_writer: log_writer,
    providers,
  }
}
