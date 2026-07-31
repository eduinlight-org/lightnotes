use dioxus::prelude::*;
use dioxus_i18n::t;

const GITHUB_URL: &str = "https://github.com/eduinlight/lightnotes";
const WEB_APP_URL: &str = "https://lightnotes.eduindev.com";

#[component]
pub fn CtaBand() -> Element {
  rsx! {
      section {
          class: "wrap pt-16 pb-[88px]",
          hr { class: "rule mb-16" }
          h2 {
              class: "max-w-[22ch] text-[clamp(28px,3vw,38px)] leading-[1.14]",
              {t!("cta-title")}
          }
          p {
              class: "mt-[22px] max-w-[56ch] text-[16px] leading-[1.68] text-copy",
              {t!("cta-description")}
          }
          div {
              class: "mt-7 flex flex-wrap gap-3",
              a { class: "btn btn-primary btn-lg", href: GITHUB_URL, {t!("cta-view-on-github")} }
              a { class: "btn btn-ghost btn-lg", href: WEB_APP_URL, "lightnotes.eduindev.com" }
          }
      }
  }
}
