use dioxus::prelude::*;
use dioxus_i18n::t;

const FEATURES: [(&str, &str, &str); 4] = [
  ("01", "features-notes-title", "features-notes-description"),
  ("02", "features-diary-title", "features-diary-description"),
  ("03", "features-markdown-title", "features-markdown-description"),
  ("04", "features-sync-title", "features-sync-description"),
];

const FEATURE_ROW_CLASS: &str = "grid grid-cols-1 gap-3.5 py-10 min-[880px]:grid-cols-[minmax(56px,120px)_minmax(0,400px)_minmax(0,1fr)] min-[880px]:items-baseline min-[880px]:gap-6 min-[880px]:gap-x-[clamp(24px,4vw,64px)]";

fn feature_rule_class(index: usize) -> &'static str {
  if index == 0 {
    ""
  } else {
    "rule-strong-top"
  }
}

#[component]
pub fn FeatureList() -> Element {
  rsx! {
      section {
          id: "features",
          class: "wrap pt-[104px] pb-10",
          span { class: "kicker", {t!("features-kicker")} }
          h2 {
              class: "max-w-[24ch] text-[clamp(30px,3.2vw,42px)]",
              {t!("features-title")}
          }
          div {
              class: "mt-14",
              for (index , (number , title_key , description_key)) in FEATURES.iter().enumerate() {
                  div {
                      class: "{FEATURE_ROW_CLASS} {feature_rule_class(index)}",
                      p { class: "font-heading text-[15px] font-medium text-accent tabular-nums", "{number}" }
                      h3 { class: "text-[25px] leading-[1.2] tracking-[-0.01em]", {t!(*title_key)} }
                      p { class: "max-w-[54ch] text-[16px] leading-[1.68] text-copy", {t!(*description_key)} }
                  }
              }
          }
      }
  }
}
