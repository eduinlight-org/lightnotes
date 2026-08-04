use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use sha2::{Digest, Sha256};

const SECRET_BYTES: usize = 32;

pub fn generate_secret() -> String {
  let bytes: [u8; SECRET_BYTES] = rand::random();
  URL_SAFE_NO_PAD.encode(bytes)
}

pub fn hash_secret(secret: &str) -> String {
  let digest = Sha256::digest(secret.as_bytes());
  digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn verify_secret(secret: &str, expected_hash: &str) -> bool {
  let actual = hash_secret(secret);

  if actual.len() != expected_hash.len() {
    return false;
  }

  actual
    .bytes()
    .zip(expected_hash.bytes())
    .fold(0u8, |acc, (left, right)| acc | (left ^ right))
    == 0
}

pub fn compose_refresh_token(token_id: &str, secret: &str) -> String {
  format!("{token_id}.{secret}")
}

pub fn split_refresh_token(token: &str) -> Option<(&str, &str)> {
  token.split_once('.')
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hashing_is_stable_and_verifiable() {
    let secret = generate_secret();
    let hash = hash_secret(&secret);

    assert_eq!(hash, hash_secret(&secret));
    assert!(verify_secret(&secret, &hash));
    assert!(!verify_secret("something-else", &hash));
  }

  #[test]
  fn secrets_are_not_reused() {
    assert_ne!(generate_secret(), generate_secret());
  }

  #[test]
  fn refresh_tokens_round_trip() {
    let composed = compose_refresh_token("token-id", "secret-value");
    let (id, secret) = split_refresh_token(&composed).expect("composed token should split");

    assert_eq!(id, "token-id");
    assert_eq!(secret, "secret-value");
    assert!(split_refresh_token("no-separator").is_none());
  }
}
