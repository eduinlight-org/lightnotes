use async_trait::async_trait;
use google_oauth::AsyncClient;

use crate::domain::ports::GoogleIdentityVerifier;
use crate::domain::user::{AuthError, GoogleIdentity};

pub struct GoogleOauthVerifier {
  clients: Vec<AsyncClient>,
}

impl GoogleOauthVerifier {
  pub fn new(client_ids: &[String]) -> Self {
    let clients = client_ids.iter().map(|client_id| AsyncClient::new(client_id)).collect();

    Self { clients }
  }
}

#[async_trait]
impl GoogleIdentityVerifier for GoogleOauthVerifier {
  async fn verify(&self, id_token: &str) -> Result<GoogleIdentity, AuthError> {
    let mut last_error = "no google client ids configured".to_string();

    for client in &self.clients {
      match client.validate_id_token(id_token).await {
        Ok(payload) => {
          if payload.email_verified != Some(true) {
            return Err(AuthError::InvalidGoogleToken("google account email is not verified".into()));
          }

          let email = payload
            .email
            .ok_or_else(|| AuthError::InvalidGoogleToken("google token carries no email".to_string()))?;

          return Ok(GoogleIdentity {
            google_sub: payload.sub,
            email,
            email_verified: true,
            name: payload.name,
            picture: payload.picture,
          });
        }
        Err(err) => last_error = err.to_string(),
      }
    }

    Err(AuthError::InvalidGoogleToken(last_error))
  }
}
