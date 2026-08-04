use crate::boot::DISMISS_SPLASH_JS;
use crate::state::{use_app_i18n, use_persisted_session, use_synced_notes, AuthState, BootState};
use dioxus::prelude::*;
use ui::components::sidebar::use_viewport_resolved;

const SPLASH_TIMEOUT_MS: u32 = 2500;

#[derive(Clone, Copy)]
pub struct AppShellState {
  pub signed_in: bool,
}

pub fn use_app_shell() -> AppShellState {
  let boot = use_context_provider(BootState::seed);
  let auth = use_context_provider(AuthState::empty);
  let store = use_synced_notes();
  use_persisted_session(auth);
  use_app_i18n(store);

  let resolved = use_viewport_resolved();
  let mut viewport_ready = boot.viewport_ready;
  use_effect(move || {
    if resolved() {
      viewport_ready.set(true);
    }
  });

  let mut timed_out = use_signal(|| false);
  use_hook(move || {
    spawn(async move {
      let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {SPLASH_TIMEOUT_MS}));"
      ))
      .await;
      viewport_ready.set(true);
      timed_out.set(true);
    });
  });

  let mut dismissed = use_signal(|| false);
  use_effect(move || {
    if dismissed() || !(boot.ready() || timed_out()) {
      return;
    }

    dismissed.set(true);
    spawn(async move {
      let _ = document::eval(DISMISS_SPLASH_JS).await;
    });
  });

  use_effect(move || {
    let theme_attr = store.theme().as_str();
    spawn(async move {
      let _ = document::eval(&format!(
        "document.documentElement.setAttribute('data-theme', '{theme_attr}');"
      ))
      .await;
    });
  });

  use_effect(move || {
    let accent = store.accent();
    spawn(async move {
      let eval = document::eval(
        "let accent = await dioxus.recv(); document.documentElement.style.setProperty('--accent', accent);",
      );
      let _ = eval.send(accent);
    });
  });

  AppShellState {
    signed_in: auth.is_signed_in(),
  }
}
