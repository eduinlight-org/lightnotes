pub struct Config {
  pub api_port: u16,
  pub mongodb_uri: String,
  pub app_env: String,
}

impl Config {
  pub fn from_env() -> Self {
    let api_port = std::env::var("API_PORT")
      .ok()
      .and_then(|value| value.parse().ok())
      .unwrap_or(4000);

    let mongodb_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    Self { api_port, mongodb_uri, app_env }
  }

  pub fn is_production(&self) -> bool {
    self.app_env == "production"
  }
}
