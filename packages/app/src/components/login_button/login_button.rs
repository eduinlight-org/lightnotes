use super::use_login_button::{use_login_button, GOOGLE_BUTTON_ID};
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_icons::lucide::LogIn;
use ui::components::button::{Button, ButtonVariant};

#[component]
pub fn LoginButton() -> Element {
  let mut state = use_login_button();
  let ready = (state.ready)();
  let failed = (state.failed)();
  let pending = (state.pending)();

  let has_status = failed || pending;

  let label = if failed {
    t!("auth-sign-in-failed")
  } else if pending {
    t!("auth-signing-in")
  } else {
    t!("action-log-in")
  };

  let label_class = if has_status { "inline" } else { "hidden sm:inline" };

  rsx! {
      div { class: "flex items-center gap-2",
          if cfg!(target_arch = "wasm32") {
              div { id: GOOGLE_BUTTON_ID, class: "min-h-[32px]" }
          }
          if !ready || !cfg!(target_arch = "wasm32") {
              Button {
                  variant: ButtonVariant::Outline,
                  disabled: cfg!(target_arch = "wasm32") || pending,
                  onclick: move |_| state.start_sign_in(),
                  LogIn { size: "16px" }
                  span { class: label_class, "{label}" }
              }
          }
      }
  }
}
