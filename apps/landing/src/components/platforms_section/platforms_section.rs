use dioxus::prelude::*;
use dioxus_i18n::t;

const MOBILE_NOTES: Asset = asset!("/assets/screenshots/mobile-notes-list.png");
const MOBILE_DIARY: Asset = asset!("/assets/screenshots/mobile-diary.png");
const MOBILE_EDITOR: Asset = asset!("/assets/screenshots/mobile-note-editor.png");

const ACCENT_SWATCHES: [&str; 6] = [
  "bg-product",
  "bg-neutral-400",
  "bg-accent-2-500",
  "bg-accent-700",
  "bg-neutral-600",
  "bg-accent-400",
];

const PHONE_FRAME_CLASS: &str = "rounded-md bg-surface p-1.5 shadow-sm";

#[component]
pub fn PlatformsSection() -> Element {
  rsx! {
      section {
          id: "platforms",
          class: "wrap grid items-center gap-8 py-[72px] min-[720px]:grid-cols-[minmax(0,5fr)_minmax(0,7fr)] min-[720px]:gap-x-[clamp(28px,5vw,88px)]",
          div {
              span { class: "kicker", {t!("platforms-kicker")} }
              h2 {
                  class: "text-[clamp(28px,3vw,36px)] leading-[1.16] tracking-[-0.012em]",
                  {t!("platforms-title")}
              }
              p {
                  class: "mt-[22px] max-w-[48ch] text-[16px] leading-[1.68] text-copy",
                  {t!("platforms-description")}
              }
              p {
                  class: "mt-[18px] max-w-[48ch] text-[16px] leading-[1.68] text-copy",
                  {t!("platforms-themes")}
              }
              div {
                  class: "mt-[22px] flex gap-2",
                  for swatch in ACCENT_SWATCHES {
                      span { class: "block h-[22px] w-[22px] rounded-full {swatch}" }
                  }
              }
          }
          div {
              class: "grid grid-cols-3 items-start gap-4",
              div {
                  class: PHONE_FRAME_CLASS,
                  img {
                      src: MOBILE_NOTES,
                      alt: t!("platforms-notes-alt"),
                      class: "w-full rounded-[6px]",
                  }
              }
              div {
                  class: "mt-7 {PHONE_FRAME_CLASS}",
                  img {
                      src: MOBILE_DIARY,
                      alt: t!("platforms-diary-alt"),
                      class: "w-full rounded-[6px]",
                  }
              }
              div {
                  class: PHONE_FRAME_CLASS,
                  img {
                      src: MOBILE_EDITOR,
                      alt: t!("platforms-editor-alt"),
                      class: "w-full rounded-[6px]",
                  }
              }
          }
      }
  }
}
