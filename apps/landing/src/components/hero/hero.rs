use crate::theme::{OUTLINED_BUTTON_CLASS, PRIMARY_BUTTON_CLASS};
use dioxus::prelude::*;
use ui::components::badge::{Badge, BadgeVariant};
use ui::components::button::{Button, ButtonSize, ButtonVariant};

const HERO_SHOT: Asset = asset!("/assets/screenshots/desktop-notes-list.png");
const GITHUB_URL: &str = "https://github.com/eduinlight/lightnotes";
const WEB_APP_URL: &str = "https://lightnotes.eduindev.com";

#[component]
pub fn Hero() -> Element {
  rsx! {
      section {
          id: "hero",
          class: "mx-auto grid max-w-6xl gap-14 px-6 pt-16 pb-4 sm:pt-24 lg:grid-cols-2 lg:items-center",
          div {
              class: "max-w-xl",
              h1 {
                  class: "text-[clamp(2rem,5vw,4rem)] font-medium leading-[1.06] tracking-tight text-white",
                  span { class: "block", "Notes that live on" }
                  span { class: "block", "your machine first" }
              }
              p {
                  class: "mt-8 max-w-[60ch] text-lg leading-relaxed text-white/80",
                  "LightNotes is a local-first notes app: a rich Markdown editor, folders, tags and a diary calendar, stored on your device and synced through an API you host yourself. One Rust codebase, built with Dioxus, shipping to web, desktop and mobile."
              }
              div {
                  class: "mt-8 flex flex-wrap gap-3",
                  a {
                      href: GITHUB_URL,
                      class: "no-underline",
                      Button { variant: ButtonVariant::Primary, size: ButtonSize::Lg, class: PRIMARY_BUTTON_CLASS, "Clone the repo" }
                  }
                  a {
                      href: WEB_APP_URL,
                      class: "no-underline",
                      Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, class: OUTLINED_BUTTON_CLASS, "Try the web app" }
                  }
              }
              div {
                  class: "mt-6 flex flex-wrap items-center gap-2",
                  Badge { variant: BadgeVariant::Outline, "MIT licensed" }
                  Badge { variant: BadgeVariant::Secondary, "Rust · Dioxus 0.7" }
                  Badge { variant: BadgeVariant::Secondary, "Self-hosted sync" }
              }
          }
          figure {
              class: "m-0 rounded-2xl bg-white/5 p-2 shadow-2xl",
              img {
                  src: HERO_SHOT,
                  alt: "LightNotes notes list on desktop",
                  class: "block w-full rounded-xl",
              }
          }
      }
  }
}
