use dioxus::prelude::*;

const STATS: [(&str, &str); 4] = [
  ("3", "Platforms, one codebase"),
  ("100%", "Offline capable"),
  ("0", "Third-party clouds"),
  ("SQLite", "On-device storage"),
];

#[component]
pub fn StatsBand() -> Element {
  rsx! {
      section {
          "aria-label": "LightNotes at a glance",
          class: "mt-16 bg-white/[0.03] py-14 sm:mt-24",
          div {
              class: "mx-auto grid max-w-6xl grid-cols-2 gap-x-6 gap-y-10 px-6 sm:grid-cols-4",
              for (value, label) in STATS {
                  div {
                      p {
                          class: "text-[clamp(2.125rem,3.4vw,3.125rem)] font-medium leading-tight text-white tabular-nums",
                          "{value}"
                      }
                      p {
                          class: "mt-3 text-xs uppercase tracking-wider text-white/65",
                          "{label}"
                      }
                  }
              }
          }
      }
  }
}
