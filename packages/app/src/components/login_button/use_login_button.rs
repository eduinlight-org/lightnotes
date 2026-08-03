use dioxus::prelude::*;

pub const GOOGLE_BUTTON_ID: &str = "google-signin-button";

#[derive(Clone, Copy)]
pub struct LoginButtonState {
  pub ready: Signal<bool>,
  pub failed: Signal<bool>,
  pub pending: Signal<bool>,
  pub start: Callback<()>,
}

impl LoginButtonState {
  pub fn start_sign_in(&mut self) {
    self.start.call(());
  }
}

#[cfg(target_arch = "wasm32")]
pub fn use_login_button() -> LoginButtonState {
  super::use_login_button_web::use_login_button()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn use_login_button() -> LoginButtonState {
  super::use_login_button_native::use_login_button()
}
