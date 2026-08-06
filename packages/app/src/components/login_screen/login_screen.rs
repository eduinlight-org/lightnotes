use crate::components::LoginButton;
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_icons::lucide::NotebookText;

#[component]
pub fn LoginScreen() -> Element {
  rsx! {
      section {
          class: "flex h-dvh w-full flex-col items-center justify-center gap-4 bg-[var(--primary-color-1)] px-6 text-center",
          NotebookText { size: "48px", stroke: "var(--accent)" }
          h1 { class: "text-2xl font-medium text-[var(--secondary-color)]", "LightNotes" }
          p {
              class: "max-w-sm text-[var(--secondary-color-5)]",
              {t!("auth-login-subtitle")}
          }
          div { class: "mt-2", LoginButton {} }
      }
  }
}
