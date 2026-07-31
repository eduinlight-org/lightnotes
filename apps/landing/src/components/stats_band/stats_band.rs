use dioxus::prelude::*;
use dioxus_i18n::t;

const STATS: [(&str, &str); 4] = [
  ("stats-platforms-value", "stats-platforms-label"),
  ("stats-offline-value", "stats-offline-label"),
  ("stats-clouds-value", "stats-clouds-label"),
  ("stats-storage-value", "stats-storage-label"),
];

#[component]
pub fn StatsBand() -> Element {
  rsx! {
      section {
          "aria-label": t!("stats-label"),
          class: "stats-band mt-24 py-16",
          div {
              class: "wrap grid grid-cols-[repeat(auto-fit,minmax(180px,1fr))] gap-x-6 gap-y-10",
              for (value_key , label_key) in STATS {
                  div {
                      p {
                          class: "-ml-[0.05em] font-heading text-[clamp(34px,3.4vw,50px)] font-medium leading-[1.1] tabular-nums",
                          {t!(value_key)}
                      }
                      p {
                          class: "mt-3 text-[13px] uppercase tracking-[0.06em] text-copy-soft",
                          {t!(label_key)}
                      }
                  }
              }
          }
      }
  }
}
