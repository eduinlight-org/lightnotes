#[derive(Debug, Clone, PartialEq)]
pub struct User {
  pub id: String,
  pub google_sub: String,
  pub email: String,
  pub email_verified: bool,
  pub name: Option<String>,
  pub picture: Option<String>,
  pub created_at_ms: i64,
  pub updated_at_ms: i64,
  pub last_login_at_ms: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoogleIdentity {
  pub google_sub: String,
  pub email: String,
  pub email_verified: bool,
  pub name: Option<String>,
  pub picture: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RefreshTokenRecord {
  pub id: String,
  pub user_id: String,
  pub token_hash: String,
  pub device_id: Option<String>,
  pub created_at_ms: i64,
  pub expires_at_ms: i64,
  pub last_used_at_ms: i64,
  pub revoked_at_ms: Option<i64>,
  pub replaced_by: Option<String>,
}

impl RefreshTokenRecord {
  pub fn is_revoked(&self) -> bool {
    self.revoked_at_ms.is_some()
  }

  pub fn is_expired(&self, now_ms: i64) -> bool {
    now_ms >= self.expires_at_ms
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketStatus {
  Pending,
  Complete,
  Failed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeAuthTicket {
  pub id: String,
  pub state: String,
  pub pkce_verifier: String,
  pub status: TicketStatus,
  pub user_id: Option<String>,
  pub access_token: Option<String>,
  pub refresh_token: Option<String>,
  pub expires_in_secs: Option<i64>,
  pub created_at_ms: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccessClaims {
  pub sub: String,
  pub exp: i64,
  pub iat: i64,
}

#[derive(Debug, Clone)]
pub enum AuthError {
  InvalidGoogleToken(String),
  InvalidToken,
  Expired,
  Revoked,
  Backend(String),
}

impl std::fmt::Display for AuthError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AuthError::InvalidGoogleToken(reason) => write!(f, "invalid google token: {reason}"),
      AuthError::InvalidToken => write!(f, "invalid token"),
      AuthError::Expired => write!(f, "token expired"),
      AuthError::Revoked => write!(f, "token revoked"),
      AuthError::Backend(reason) => write!(f, "auth backend error: {reason}"),
    }
  }
}

impl std::error::Error for AuthError {}

impl From<super::ports::RepositoryError> for AuthError {
  fn from(value: super::ports::RepositoryError) -> Self {
    AuthError::Backend(value.to_string())
  }
}
