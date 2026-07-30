use dioxus::prelude::*;
use dioxus_icons::lucide::NotebookPen;

const FOOTER_LINK_CLASS: &str = "text-white/60 no-underline hover:text-white";
const FOOTER_HEADING_CLASS: &str = "text-xs uppercase tracking-wider text-white/40";

const PRODUCT_LINKS: [(&str, &str); 4] = [
  ("#features", "Features"),
  ("#platforms", "Platforms"),
  ("#selfhost", "Self-hosting"),
  ("#download", "Download"),
];

const SOURCE_LINKS: [(&str, &str); 3] = [
  ("https://github.com/eduinlight/lightnotes", "Repository"),
  ("https://github.com/eduinlight/lightnotes/blob/main/LICENSE", "MIT license"),
  ("https://dioxuslabs.com/", "Dioxus"),
];

#[component]
pub fn Footer() -> Element {
  rsx! {
      footer {
          class: "mt-auto border-t border-white/10",
          div {
              class: "mx-auto flex max-w-6xl flex-wrap justify-between gap-8 px-6 py-12 text-[13.5px] leading-loose text-white/60",
              div {
                  class: "grid max-w-[34ch] gap-2",
                  span {
                      class: "flex items-center gap-2 text-base font-medium text-white",
                      NotebookPen { size: "18px", stroke: "#9a8fe0" }
                      "LightNotes"
                  }
                  span { "A local-first notes app built with Dioxus 0.7. MIT licensed." }
              }
              div {
                  class: "grid gap-1.5",
                  span { class: FOOTER_HEADING_CLASS, "Product" }
                  for (href , label) in PRODUCT_LINKS {
                      a { href, class: FOOTER_LINK_CLASS, "{label}" }
                  }
              }
              div {
                  class: "grid gap-1.5",
                  span { class: FOOTER_HEADING_CLASS, "Source" }
                  for (href , label) in SOURCE_LINKS {
                      a { href, class: FOOTER_LINK_CLASS, "{label}" }
                  }
              }
              div {
                  class: "grid gap-1.5",
                  span { class: FOOTER_HEADING_CLASS, "Contact" }
                  a { href: "mailto:eduinlight@gmail.com", class: FOOTER_LINK_CLASS, "eduinlight@gmail.com" }
              }
          }
      }
  }
}
