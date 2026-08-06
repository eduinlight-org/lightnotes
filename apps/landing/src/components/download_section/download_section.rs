use crate::config;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Clone, Copy)]
enum Platform {
  MacOs,
  Windows,
  Linux,
  Ios,
  Android,
}

const DOWNLOADS: [(Platform, &str, &str, &str); 5] = [
  (Platform::MacOs, "macOS", "download-macos-description", "download-status-coming-soon"),
  (Platform::Windows, "Windows", "download-windows-description", "download-status-coming-soon"),
  (Platform::Linux, "Linux", "download-linux-description", "download-status-coming-soon"),
  (Platform::Ios, "iOS", "download-ios-description", "download-status-in-review"),
  (Platform::Android, "Android", "download-android-description", "download-status-in-review"),
];

fn platform_icon_class(platform: Platform) -> &'static str {
  match platform {
    Platform::MacOs => "ph ph-apple-logo",
    Platform::Windows => "ph ph-windows-logo",
    Platform::Linux => "ph ph-linux-logo",
    Platform::Ios => "ph ph-app-store-logo",
    Platform::Android => "ph ph-google-play-logo",
  }
}

#[component]
pub fn DownloadSection() -> Element {
  rsx! {
      section {
          id: "download",
          class: "wrap pt-[72px] pb-10",
          span { class: "kicker", {t!("download-kicker")} }
          div {
              class: "flex flex-wrap items-baseline justify-between gap-x-8 gap-y-4",
              h2 {
                  class: "max-w-[24ch] text-[clamp(28px,3vw,36px)] leading-[1.16] tracking-[-0.012em]",
                  {t!("download-title")}
              }
              span { class: "tag tag-outline", {t!("download-badge")} }
          }
          div {
              class: "mt-10 grid grid-cols-[repeat(auto-fit,minmax(230px,1fr))] gap-4",
              for (platform , title , description_key , status_key) in DOWNLOADS {
                  div {
                      class: "card elev-sm gap-2.5 px-5 py-[18px]",
                      i { class: "{platform_icon_class(platform)} text-2xl leading-none text-accent" }
                      span { class: "card-title mt-1", "{title}" }
                      span { class: "text-[13px] text-copy-faint", {t!(description_key)} }
                      span { class: "tag tag-neutral mt-1.5 self-start", {t!(status_key)} }
                  }
              }
          }
          p {
              class: "mt-[26px] max-w-[62ch] text-[15px] leading-[1.7] text-copy-mid",
              {t!("download-note-before")}
              " "
              code { class: "font-mono text-[14px] text-accent-300", "make" }
              {t!("download-note-after")}
          }
          div {
              class: "mt-[22px] flex flex-wrap gap-3",
              a { class: "btn btn-primary btn-md", href: config::APP_URL, {t!("download-open-web-app")} }
              a { class: "btn btn-secondary btn-md", href: config::github_build_url(), {t!("download-build-from-source")} }
          }
      }
  }
}
