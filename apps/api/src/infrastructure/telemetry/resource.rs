use opentelemetry::KeyValue;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::resource::{
  DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_INSTANCE_ID, SERVICE_VERSION,
};

use crate::infrastructure::config::Config;

pub fn build(config: &Config) -> Resource {
  let instance_id = std::env::var("HOSTNAME")
    .ok()
    .map(|value| value.trim().to_string())
    .filter(|value| !value.is_empty())
    .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

  Resource::builder()
    .with_service_name(config.service_name.clone())
    .with_attributes([
      KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
      KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, config.app_env.clone()),
      KeyValue::new(SERVICE_INSTANCE_ID, instance_id),
    ])
    .build()
}
