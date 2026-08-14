use opentelemetry::global;
use opentelemetry_otlp::{LogExporter, MetricExporter, SpanExporter};
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::SdkTracerProvider;

use crate::infrastructure::config::Config;

use super::{metrics, process, resource};

#[derive(Default)]
pub struct Providers {
  pub tracer: Option<SdkTracerProvider>,
  pub meter: Option<SdkMeterProvider>,
  pub logger: Option<SdkLoggerProvider>,
}

impl Providers {
  pub fn shutdown(&self) {
    if let Some(tracer) = &self.tracer {
      if let Err(error) = tracer.shutdown() {
        eprintln!("otel: tracer provider shutdown failed: {error}");
      }
    }

    if let Some(meter) = &self.meter {
      if let Err(error) = meter.shutdown() {
        eprintln!("otel: meter provider shutdown failed: {error}");
      }
    }

    if let Some(logger) = &self.logger {
      if let Err(error) = logger.shutdown() {
        eprintln!("otel: logger provider shutdown failed: {error}");
      }
    }
  }
}

pub fn init(config: &Config) -> Providers {
  if config.otlp_endpoint.is_none() {
    return Providers::default();
  }

  let resource = resource::build(config);

  let tracer = match SpanExporter::builder().with_http().build() {
    Ok(exporter) => Some(
      SdkTracerProvider::builder()
        .with_resource(resource.clone())
        .with_batch_exporter(exporter)
        .build(),
    ),
    Err(error) => {
      eprintln!("otel: span exporter unavailable, traces disabled: {error}");
      None
    }
  };

  let meter = match MetricExporter::builder().with_http().build() {
    Ok(exporter) => Some(
      SdkMeterProvider::builder()
        .with_resource(resource.clone())
        .with_periodic_exporter(exporter)
        .build(),
    ),
    Err(error) => {
      eprintln!("otel: metric exporter unavailable, metrics disabled: {error}");
      None
    }
  };

  let logger = match LogExporter::builder().with_http().build() {
    Ok(exporter) => Some(
      SdkLoggerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter)
        .build(),
    ),
    Err(error) => {
      eprintln!("otel: log exporter unavailable, log export disabled: {error}");
      None
    }
  };

  if let Some(tracer) = &tracer {
    global::set_tracer_provider(tracer.clone());
  }

  if let Some(meter) = &meter {
    global::set_meter_provider(meter.clone());
    process::register(metrics::METER_NAME);
  }

  global::set_text_map_propagator(TraceContextPropagator::new());

  Providers {
    tracer,
    meter,
    logger,
  }
}
