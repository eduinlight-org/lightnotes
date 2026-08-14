use mongodb::event::command::CommandEvent;
use mongodb::event::EventHandler;
use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use opentelemetry::KeyValue;
use opentelemetry_semantic_conventions::attribute::DB_OPERATION_NAME;

use super::telemetry::AppMetrics;

const IGNORED_COMMANDS: [&str; 5] = ["hello", "isMaster", "ping", "endSessions", "buildInfo"];

pub async fn connect(uri: &str, metrics: &AppMetrics) -> Database {
  let mut options = ClientOptions::parse(uri).await.expect("failed to parse MONGODB_URI");
  options.command_event_handler = Some(command_metrics_handler(metrics.clone()));

  let client = Client::with_options(options).expect("failed to connect to MongoDB");
  client.default_database().expect("MONGODB_URI must include a default database name")
}

fn command_metrics_handler(metrics: AppMetrics) -> EventHandler<CommandEvent> {
  EventHandler::callback(move |event| {
    let (command_name, duration, outcome) = match event {
      CommandEvent::Succeeded(succeeded) => (succeeded.command_name, succeeded.duration, "ok"),
      CommandEvent::Failed(failed) => (failed.command_name, failed.duration, "error"),
      _ => return,
    };

    if IGNORED_COMMANDS.contains(&command_name.as_str()) {
      return;
    }

    metrics.mongo_command_duration.record(
      duration.as_secs_f64(),
      &[
        KeyValue::new(DB_OPERATION_NAME, command_name),
        KeyValue::new("db.response.status", outcome),
      ],
    );
  })
}
