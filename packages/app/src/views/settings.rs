use crate::components::SettingsPanel;
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
  rsx! {
      div { class: "h-full min-h-0 overflow-y-auto",
          div { class: "mx-auto max-w-2xl px-6 py-8 md:px-8",
              h1 { class: "mb-7 text-[28px] font-medium text-[var(--secondary-color)]", "Settings" }
              SettingsPanel {}
          }
      }
  }
}
