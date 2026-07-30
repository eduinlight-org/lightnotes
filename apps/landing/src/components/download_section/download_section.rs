use crate::theme::PRIMARY_BUTTON_CLASS;
use dioxus::prelude::*;
use dioxus_icons::lucide::{AppWindow, Apple, Smartphone, TabletSmartphone, Terminal};
use ui::components::badge::{Badge, BadgeVariant};
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::card::{Card, CardContent, CardHeader, CardTitle};

const GITHUB_BUILD_URL: &str = "https://github.com/eduinlight/lightnotes#building-for-release";
const WEB_APP_URL: &str = "https://lightnotes.eduindev.com";

#[derive(Clone, Copy)]
enum Platform {
  MacOs,
  Windows,
  Linux,
  Ios,
  Android,
}

const DOWNLOADS: [(Platform, &str, &str, &str); 5] = [
  (Platform::MacOs, "macOS", "Universal .dmg — Apple silicon & Intel", "Coming soon"),
  (Platform::Windows, "Windows", "Signed .exe installer, x86-64", "Coming soon"),
  (Platform::Linux, "Linux", "AppImage and .deb packages", "Coming soon"),
  (Platform::Ios, "iOS", "App Store — iPhone and iPad", "In review"),
  (Platform::Android, "Android", "Google Play — phones and tablets", "In review"),
];

fn platform_icon(platform: Platform) -> Element {
  match platform {
    Platform::MacOs => rsx! { Apple { size: "24px", stroke: "#9a8fe0" } },
    Platform::Windows => rsx! { AppWindow { size: "24px", stroke: "#9a8fe0" } },
    Platform::Linux => rsx! { Terminal { size: "24px", stroke: "#9a8fe0" } },
    Platform::Ios => rsx! { Smartphone { size: "24px", stroke: "#9a8fe0" } },
    Platform::Android => rsx! { TabletSmartphone { size: "24px", stroke: "#9a8fe0" } },
  }
}

#[component]
pub fn DownloadSection() -> Element {
  rsx! {
      section {
          id: "download",
          class: "mx-auto max-w-6xl px-6 py-20",
          div {
              class: "flex flex-wrap items-baseline justify-between gap-4",
              span { class: "block text-xs uppercase tracking-wider text-[#9a8fe0]", "Download" }
          }
          div {
              class: "mt-3 flex flex-wrap items-baseline justify-between gap-4",
              h2 {
                  class: "max-w-[24ch] text-[clamp(1.75rem,3vw,2.25rem)] font-medium leading-tight tracking-tight text-white",
                  "Get the app on your platform."
              }
              Badge { variant: BadgeVariant::Outline, "Builds in progress" }
          }
          div {
              class: "mt-10 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3",
              for (platform , title , description , status) in DOWNLOADS {
                  Card {
                      CardHeader { {platform_icon(platform)} }
                      CardContent {
                          div {
                              class: "flex flex-col items-start gap-2",
                              CardTitle { "{title}" }
                              span { class: "text-sm text-white/60", "{description}" }
                              Badge { variant: BadgeVariant::Secondary, "{status}" }
                          }
                      }
                  }
              }
          }
          p {
              class: "mt-8 max-w-[62ch] leading-relaxed text-white/70",
              "Packaged builds aren't out yet. Until they are, the web app runs today and every desktop and mobile target builds from source with a single "
              code { class: "font-mono text-sm text-[#c2b9f0]", "make" }
              " command."
          }
          div {
              class: "mt-6 flex flex-wrap gap-3",
              a {
                  href: WEB_APP_URL,
                  class: "no-underline",
                  Button { variant: ButtonVariant::Primary, size: ButtonSize::Sm, class: PRIMARY_BUTTON_CLASS, "Open the web app" }
              }
              a {
                  href: GITHUB_BUILD_URL,
                  class: "no-underline",
                  Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Build from source" }
              }
          }
      }
  }
}
