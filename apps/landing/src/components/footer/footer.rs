use crate::config;
use dioxus::prelude::*;
use dioxus_i18n::t;

const BRAND_ICON: Asset = asset!("/assets/notes-icon.png");

const FOOTER_HEADING_CLASS: &str = "text-[12px] uppercase tracking-[0.06em] text-copy-ghost";

const PRODUCT_LINKS: [(&str, &str); 4] = [
  ("#features", "nav-features"),
  ("#platforms", "nav-platforms"),
  ("#selfhost", "nav-self-hosting"),
  ("#download", "nav-download"),
];

fn source_links() -> [(String, &'static str); 3] {
  [
    (config::GITHUB_URL.to_string(), "footer-repository"),
    (config::github_license_url(), "footer-license"),
    (config::DIOXUS_URL.to_string(), "footer-dioxus"),
  ]
}

#[component]
pub fn Footer() -> Element {
  rsx! {
      footer {
          class: "rule-top mt-auto",
          div {
              class: "wrap flex flex-wrap items-start justify-between gap-8 pt-10 pb-14 text-[13.5px] leading-[1.9] text-copy-faint",
              div {
                  class: "grid max-w-[34ch] gap-2",
                  span {
                      class: "flex items-center gap-[9px] font-heading text-[16px] font-medium text-text",
                      img { src: BRAND_ICON, alt: "", class: "h-[22px] w-[22px] rounded-[6px]" }
                      {t!("brand-name")}
                  }
                  span { {t!("footer-tagline")} }
              }
              div {
                  class: "grid gap-1.5",
                  span { class: FOOTER_HEADING_CLASS, {t!("footer-product")} }
                  for (href , label_key) in PRODUCT_LINKS {
                      a { href, class: "no-underline", {t!(label_key)} }
                  }
              }
              div {
                  class: "grid gap-1.5",
                  span { class: FOOTER_HEADING_CLASS, {t!("footer-source")} }
                  for (href , label_key) in source_links() {
                      a { href, class: "no-underline", {t!(label_key)} }
                  }
              }
              div {
                  class: "grid gap-1.5",
                  span { class: FOOTER_HEADING_CLASS, {t!("footer-contact")} }
                  a { href: config::contact_mailto(), class: "no-underline", {config::CONTACT_EMAIL} }
              }
          }
      }
  }
}
