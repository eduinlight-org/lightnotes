use async_trait::async_trait;
use serde::Deserialize;

use crate::domain::ports::GoogleCodeExchanger;
use crate::domain::user::AuthError;

const AUTHORIZE_ENDPOINT: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";
const SCOPES: &str = "openid email profile";

#[derive(Debug, Deserialize)]
struct TokenResponse {
  id_token: Option<String>,
  error_description: Option<String>,
  error: Option<String>,
}

pub struct GoogleCodeClient {
  http: reqwest::Client,
  client_id: String,
  client_secret: String,
  redirect_uri: String,
}

impl GoogleCodeClient {
  pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
    Self {
      http: reqwest::Client::new(),
      client_id,
      client_secret,
      redirect_uri,
    }
  }
}

fn encode(value: &str) -> String {
  value
    .bytes()
    .map(|byte| match byte {
      b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (byte as char).to_string(),
      _ => format!("%{byte:02X}"),
    })
    .collect()
}

#[async_trait]
impl GoogleCodeExchanger for GoogleCodeClient {
  async fn authorize_url(&self, state: &str, pkce_challenge: &str) -> String {
    format!(
      "{AUTHORIZE_ENDPOINT}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256&access_type=offline&prompt=select_account",
      encode(&self.client_id),
      encode(&self.redirect_uri),
      encode(SCOPES),
      encode(state),
      encode(pkce_challenge),
    )
  }

  async fn exchange_code(&self, code: &str, pkce_verifier: &str) -> Result<String, AuthError> {
    let params = [
      ("code", code),
      ("client_id", self.client_id.as_str()),
      ("client_secret", self.client_secret.as_str()),
      ("redirect_uri", self.redirect_uri.as_str()),
      ("grant_type", "authorization_code"),
      ("code_verifier", pkce_verifier),
    ];

    let response = self
      .http
      .post(TOKEN_ENDPOINT)
      .form(&params)
      .send()
      .await
      .map_err(|err| AuthError::Backend(err.to_string()))?;

    let payload = response
      .json::<TokenResponse>()
      .await
      .map_err(|err| AuthError::Backend(err.to_string()))?;

    if let Some(error) = payload.error {
      let detail = payload.error_description.unwrap_or(error);
      return Err(AuthError::InvalidGoogleToken(detail));
    }

    payload
      .id_token
      .ok_or_else(|| AuthError::InvalidGoogleToken("google token response carried no id_token".to_string()))
  }
}
