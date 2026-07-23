use dioxus::prelude::*;
use ui::components::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn CtaBand() -> Element {
  rsx! {
      section {
          class: "mx-auto flex max-w-3xl flex-col items-center gap-6 px-6 py-24 text-center",
          h2 {
              class: "text-3xl font-bold text-white sm:text-4xl",
              "Ready to get organized?"
          }
          p {
              class: "max-w-lg text-lg text-[#a1a1a1]",
              "Start writing text notes and tracking todo lists in seconds. No setup required."
          }
          Button { variant: ButtonVariant::Primary, size: ButtonSize::Lg, "Get started for free" }
      }
  }
}
