const MIN_JWT_SECRET_BYTES: usize = 32;

pub struct Config {
  pub api_port: u16,
  pub mongodb_uri: String,
  pub app_env: String,
  pub google_client_ids: Vec<String>,
  pub google_client_secret: String,
  pub google_redirect_uri: String,
  pub jwt_secret: String,
  pub access_token_ttl_secs: i64,
  pub refresh_token_ttl_secs: i64,
}

impl Config {
  pub fn from_env() -> Self {
    let api_port = std::env::var("API_PORT")
      .ok()
      .and_then(|value| value.parse().ok())
      .unwrap_or(4000);

    let mongodb_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    let google_client_ids: Vec<String> = std::env::var("GOOGLE_CLIENT_IDS")
      .expect("GOOGLE_CLIENT_IDS must be set")
      .split(',')
      .map(|value| value.trim().to_string())
      .filter(|value| !value.is_empty())
      .collect();

    assert!(
      !google_client_ids.is_empty(),
      "GOOGLE_CLIENT_IDS must list at least one client id"
    );

    let google_client_secret = std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default();

    let google_redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")
      .unwrap_or_else(|_| format!("http://localhost:{api_port}/api/v1/auth/native/callback"));

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    assert!(
      jwt_secret.len() >= MIN_JWT_SECRET_BYTES,
      "JWT_SECRET must be at least {MIN_JWT_SECRET_BYTES} bytes"
    );

    let access_token_ttl_secs = std::env::var("ACCESS_TOKEN_TTL_SECS")
      .ok()
      .and_then(|value| value.parse().ok())
      .unwrap_or(900);

    let refresh_token_ttl_secs = std::env::var("REFRESH_TOKEN_TTL_SECS")
      .ok()
      .and_then(|value| value.parse().ok())
      .unwrap_or(2_592_000);

    Self {
      api_port,
      mongodb_uri,
      app_env,
      google_client_ids,
      google_client_secret,
      google_redirect_uri,
      jwt_secret,
      access_token_ttl_secs,
      refresh_token_ttl_secs,
    }
  }

  pub fn is_production(&self) -> bool {
    self.app_env == "production"
  }
}
