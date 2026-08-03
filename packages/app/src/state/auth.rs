use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use sync_dto::{AuthTokensDto, UserDto};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuthStatus {
  #[default]
  Unknown,
  SignedOut,
  SignedIn,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistedSession {
  pub user: UserDto,
  pub tokens: AuthTokensDto,
}

#[derive(Clone, Copy)]
pub struct AuthState {
  pub user: Signal<Option<UserDto>>,
  pub tokens: Signal<Option<AuthTokensDto>>,
  pub status: Signal<AuthStatus>,
  pub generation: Signal<u64>,
}

impl AuthState {
  pub fn empty() -> Self {
    Self {
      user: Signal::new(None),
      tokens: Signal::new(None),
      status: Signal::new(AuthStatus::Unknown),
      generation: Signal::new(0),
    }
  }

  pub fn user(&self) -> Option<UserDto> {
    (self.user)()
  }

  pub fn tokens(&self) -> Option<AuthTokensDto> {
    (self.tokens)()
  }

  pub fn status(&self) -> AuthStatus {
    (self.status)()
  }

  pub fn generation(&self) -> u64 {
    (self.generation)()
  }

  pub fn is_signed_in(&self) -> bool {
    self.status() == AuthStatus::SignedIn
  }

  pub fn display_name(&self) -> String {
    self
      .user()
      .map(|user| user.name.unwrap_or(user.email))
      .unwrap_or_default()
  }

  pub fn sign_in(&mut self, user: UserDto, tokens: AuthTokensDto) {
    self.user.set(Some(user));
    self.tokens.set(Some(tokens));
    self.status.set(AuthStatus::SignedIn);
    self.generation += 1;
  }

  pub fn restore(&mut self, session: PersistedSession) {
    self.user.set(Some(session.user));
    self.tokens.set(Some(session.tokens));
    self.status.set(AuthStatus::SignedIn);
  }

  pub fn set_tokens(&mut self, tokens: AuthTokensDto) {
    self.tokens.set(Some(tokens));
  }

  pub fn settle_restore(&mut self) {
    if self.status() == AuthStatus::Unknown {
      self.status.set(AuthStatus::SignedOut);
    }
  }

  pub fn sign_out(&mut self) {
    self.user.set(None);
    self.tokens.set(None);
    self.status.set(AuthStatus::SignedOut);
    self.generation += 1;
  }
}

pub fn use_auth() -> AuthState {
  use_context()
}
