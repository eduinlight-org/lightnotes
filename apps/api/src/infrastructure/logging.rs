use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

pub fn init(is_production: bool) -> Option<WorkerGuard> {
  let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug"));

  if !is_production {
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
    return None;
  }

  let log_dir = std::env::var("API_LOG_DIR").unwrap_or_else(|_| "logs".to_string());
  std::fs::create_dir_all(&log_dir).expect("failed to create log directory");

  let file_appender = tracing_appender::rolling::daily(log_dir, "api.log");
  let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

  tracing_subscriber::fmt()
    .with_env_filter(env_filter)
    .with_writer(non_blocking)
    .with_ansi(false)
    .json()
    .init();

  Some(guard)
}
