use crate::config;
use dioxus::prelude::*;
use dioxus_i18n::t;

const HERO_SHOT: Asset = asset!("/assets/screenshots/desktop-notes-list.png");

#[component]
pub fn Hero() -> Element {
  rsx! {
      section {
          id: "hero",
          class: "wrap grid gap-14 pt-24",
          div {
              class: "max-w-[780px]",
              h1 {
                  class: "-ml-[0.06em] text-[clamp(36px,5vw,60px)] leading-[1.06] tracking-[-0.02em]",
                  span { class: "block", {t!("hero-title-line-1")} }
                  span { class: "block", {t!("hero-title-line-2")} }
              }
              p {
                  class: "mt-[30px] max-w-[60ch] text-[18px] leading-[1.62] text-copy-strong",
                  {t!("hero-description")}
              }
              div {
                  class: "mt-[30px] flex flex-wrap gap-3",
                  a { class: "btn btn-primary btn-lg", href: config::GITHUB_URL, {t!("hero-clone-repo")} }
                  a { class: "btn btn-secondary btn-lg", href: config::APP_URL, {t!("hero-try-web-app")} }
              }
              div {
                  class: "mt-[26px] flex flex-wrap items-center gap-2.5",
                  span { class: "tag tag-outline", {t!("hero-tag-license")} }
                  span { class: "tag tag-neutral", {t!("hero-tag-stack")} }
                  span { class: "tag tag-neutral", {t!("hero-tag-sync")} }
              }
          }
          figure {
              class: "overflow-hidden rounded-lg bg-surface p-2 shadow-md",
              img {
                  src: HERO_SHOT,
                  alt: t!("hero-screenshot-alt"),
                  class: "w-full rounded-[10px]",
              }
          }
      }
  }
}
