use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::domain::ports::TokenIssuer;
use crate::domain::user::{AccessClaims, AuthError};

const ISSUER: &str = "lightnotes-api";
const AUDIENCE: &str = "lightnotes-app";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  sub: String,
  iss: String,
  aud: String,
  iat: i64,
  exp: i64,
}

pub struct HmacTokenIssuer {
  encoding_key: EncodingKey,
  decoding_key: DecodingKey,
  validation: Validation,
  access_ttl_secs: i64,
}

impl HmacTokenIssuer {
  pub fn new(secret: &str, access_ttl_secs: i64) -> Self {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[ISSUER]);
    validation.set_audience(&[AUDIENCE]);

    Self {
      encoding_key: EncodingKey::from_secret(secret.as_bytes()),
      decoding_key: DecodingKey::from_secret(secret.as_bytes()),
      validation,
      access_ttl_secs,
    }
  }
}

impl TokenIssuer for HmacTokenIssuer {
  fn issue_access(&self, user_id: &str, now_ms: i64) -> Result<String, AuthError> {
    let issued_at = now_ms / 1000;

    let claims = Claims {
      sub: user_id.to_string(),
      iss: ISSUER.to_string(),
      aud: AUDIENCE.to_string(),
      iat: issued_at,
      exp: issued_at + self.access_ttl_secs,
    };

    encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key).map_err(|err| AuthError::Backend(err.to_string()))
  }

  fn verify_access(&self, token: &str) -> Result<AccessClaims, AuthError> {
    let decoded = decode::<Claims>(token, &self.decoding_key, &self.validation).map_err(|err| match err.kind() {
      jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::Expired,
      _ => AuthError::InvalidToken,
    })?;

    Ok(AccessClaims {
      sub: decoded.claims.sub,
      exp: decoded.claims.exp,
      iat: decoded.claims.iat,
    })
  }

  fn access_ttl_secs(&self) -> i64 {
    self.access_ttl_secs
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const SECRET: &str = "test-secret-that-is-long-enough-for-hs256";

  fn now_ms() -> i64 {
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .expect("system clock before unix epoch")
      .as_millis() as i64
  }

  #[test]
  fn issued_tokens_verify_back_to_the_same_user() {
    let issuer = HmacTokenIssuer::new(SECRET, 900);
    let token = issuer.issue_access("user-1", now_ms()).expect("token should be issued");
    let claims = issuer.verify_access(&token).expect("token should verify");

    assert_eq!(claims.sub, "user-1");
    assert_eq!(claims.exp - claims.iat, 900);
  }

  #[test]
  fn expired_tokens_are_rejected() {
    let issuer = HmacTokenIssuer::new(SECRET, 900);
    let token = issuer.issue_access("user-1", 0).expect("token should be issued");

    assert!(matches!(issuer.verify_access(&token), Err(AuthError::Expired)));
  }

  #[test]
  fn tokens_signed_with_another_secret_are_rejected() {
    let issuer = HmacTokenIssuer::new(SECRET, 900);
    let attacker = HmacTokenIssuer::new("a-completely-different-secret-value", 900);
    let token = attacker.issue_access("user-1", now_ms()).expect("token should be issued");

    assert!(matches!(issuer.verify_access(&token), Err(AuthError::InvalidToken)));
  }

  #[test]
  fn tampered_tokens_are_rejected() {
    let issuer = HmacTokenIssuer::new(SECRET, 900);
    let token = issuer.issue_access("user-1", now_ms()).expect("token should be issued");
    let mut tampered = token.clone();
    tampered.pop();
    tampered.push(if token.ends_with('a') { 'b' } else { 'a' });

    assert!(matches!(issuer.verify_access(&tampered), Err(AuthError::InvalidToken)));
  }
}
