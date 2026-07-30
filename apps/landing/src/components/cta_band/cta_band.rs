use crate::theme::PRIMARY_BUTTON_CLASS;
use dioxus::prelude::*;
use ui::components::button::{Button, ButtonSize, ButtonVariant};

const GITHUB_URL: &str = "https://github.com/eduinlight/lightnotes";
const WEB_APP_URL: &str = "https://lightnotes.eduindev.com";

#[component]
pub fn CtaBand() -> Element {
  rsx! {
      section {
          class: "mx-auto max-w-6xl px-6 py-16 sm:py-20",
          hr { class: "mb-16 h-px border-0 bg-white/10" }
          h2 {
              class: "max-w-[22ch] text-[clamp(1.75rem,3vw,2.375rem)] font-medium leading-tight tracking-tight text-white",
              "Keep your notes. Keep your server."
          }
          p {
              class: "mt-6 max-w-[56ch] leading-relaxed text-white/80",
              "LightNotes is MIT licensed and open source. Clone it, build it for your platform, and point it at your own API."
          }
          div {
              class: "mt-7 flex flex-wrap gap-3",
              a {
                  href: GITHUB_URL,
                  class: "no-underline",
                  Button { variant: ButtonVariant::Primary, size: ButtonSize::Lg, class: PRIMARY_BUTTON_CLASS, "View on GitHub" }
              }
              a {
                  href: WEB_APP_URL,
                  class: "no-underline",
                  Button { variant: ButtonVariant::Ghost, size: ButtonSize::Lg, "lightnotes.eduindev.com" }
              }
          }
      }
  }
}
