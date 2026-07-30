use crate::theme::PRIMARY_BUTTON_CLASS;
use dioxus::prelude::*;
use ui::components::button::{Button, ButtonSize, ButtonVariant};

const SETUP_GUIDE_URL: &str = "https://github.com/eduinlight/lightnotes#getting-started";

const COMMANDS: [(&str, &str); 4] = [
  ("cp .env.dist .env", ""),
  ("make docker-up", "mongo + admin UI"),
  ("make api-dev", "sync API"),
  ("make app-web-dev", "or app-desktop-dev"),
];

#[component]
pub fn SelfHostSection() -> Element {
  rsx! {
      section {
          id: "selfhost",
          class: "mx-auto grid max-w-6xl items-start gap-10 px-6 py-20 lg:grid-cols-2 lg:gap-16",
          div {
              span { class: "block text-xs uppercase tracking-wider text-[#9a8fe0]", "Self-hosting" }
              h2 {
                  class: "mt-3 text-[clamp(1.75rem,3vw,2.25rem)] font-medium leading-tight tracking-tight text-white",
                  "Run the whole thing yourself."
              }
              p {
                  class: "mt-6 max-w-[48ch] leading-relaxed text-white/80",
                  "You run the axum API next to the clients: REST and SSE over MongoDB, started with Docker Compose. Stable Rust and the Dioxus CLI are the only prerequisites — Tailwind compiles inside "
                  code { class: "font-mono text-sm text-[#c2b9f0]", "dx build" }
                  ", so there is no Node step. A hosted cloud version is in development."
              }
              div {
                  class: "mt-6 flex flex-wrap gap-3",
                  a {
                      href: SETUP_GUIDE_URL,
                      class: "no-underline",
                      Button { variant: ButtonVariant::Primary, size: ButtonSize::Sm, class: PRIMARY_BUTTON_CLASS, "Read the setup guide" }
                  }
              }
          }
          div {
              class: "grid gap-1 rounded-xl bg-white/5 px-6 py-5 font-mono text-[13.5px] leading-loose text-[#d4d4d4]",
              for (command , comment) in COMMANDS {
                  div {
                      span { class: "text-white/40", "$ " }
                      "{command}"
                      if !comment.is_empty() {
                          span { class: "text-white/40", "  # {comment}" }
                      }
                  }
              }
          }
      }
  }
}
