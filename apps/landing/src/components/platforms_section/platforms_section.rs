use dioxus::prelude::*;

const MOBILE_NOTES: Asset = asset!("/assets/screenshots/mobile-notes-list.png");
const MOBILE_DIARY: Asset = asset!("/assets/screenshots/mobile-diary.png");
const MOBILE_EDITOR: Asset = asset!("/assets/screenshots/mobile-note-editor.png");

const ACCENT_SWATCHES: [&str; 6] = ["#9a8fe0", "#71717a", "#2dd4bf", "#7c6fd1", "#52525b", "#c2b9f0"];

#[component]
pub fn PlatformsSection() -> Element {
  rsx! {
      section {
          id: "platforms",
          class: "mx-auto grid max-w-6xl items-center gap-10 px-6 py-20 lg:grid-cols-[5fr_7fr] lg:gap-16",
          div {
              span { class: "block text-xs uppercase tracking-wider text-[#9a8fe0]", "Every platform" }
              h2 {
                  class: "mt-3 text-[clamp(1.75rem,3vw,2.25rem)] font-medium leading-tight tracking-tight text-white",
                  "Desktop sidebar, mobile tab bar, same notes."
              }
              p {
                  class: "mt-6 max-w-[48ch] leading-relaxed text-white/80",
                  "Routing, views and components live in one shared package; each platform shell is thin. The layout adapts on its own — a persistent sidebar and top bar on desktop, a compact header with bottom tabs on mobile."
              }
              p {
                  class: "mt-5 max-w-[48ch] leading-relaxed text-white/80",
                  "Dark and light themes, with six accent colors carried through highlights, links and controls."
              }
              div {
                  class: "mt-6 flex gap-2",
                  for color in ACCENT_SWATCHES {
                      span {
                          class: "block h-[22px] w-[22px] rounded-full",
                          style: "background: {color};",
                      }
                  }
              }
          }
          div {
              class: "grid grid-cols-3 items-start gap-4",
              div {
                  class: "rounded-xl bg-white/5 p-1.5 shadow-lg",
                  img {
                      src: MOBILE_NOTES,
                      alt: "LightNotes notes list on mobile",
                      class: "block w-full rounded-md",
                  }
              }
              div {
                  class: "mt-7 rounded-xl bg-white/5 p-1.5 shadow-lg",
                  img {
                      src: MOBILE_DIARY,
                      alt: "LightNotes diary calendar on mobile",
                      class: "block w-full rounded-md",
                  }
              }
              div {
                  class: "rounded-xl bg-white/5 p-1.5 shadow-lg",
                  img {
                      src: MOBILE_EDITOR,
                      alt: "LightNotes note editor on mobile",
                      class: "block w-full rounded-md",
                  }
              }
          }
      }
  }
}
