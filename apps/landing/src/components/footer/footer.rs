use dioxus::prelude::*;

const FOOTER_LINK_CLASS: &str = "text-[#a1a1a1] no-underline hover:text-white";

#[component]
pub fn Footer() -> Element {
  rsx! {
      footer {
          class: "mt-auto border-t border-white/10 bg-[#0b0d12] px-6 py-8",
          div {
              class: "mx-auto flex max-w-5xl flex-col items-center justify-between gap-4 sm:flex-row",
              div {
                  class: "flex items-center gap-2 text-white",
                  span { "📝" }
                  span { class: "font-semibold", "Notes" }
              }
              div {
                  class: "flex gap-6 text-sm",
                  a { href: "https://dioxuslabs.com/learn/0.7/", class: FOOTER_LINK_CLASS, "Docs" }
                  a { href: "https://github.com/dioxus-community/", class: FOOTER_LINK_CLASS, "Community" }
                  a { href: "https://discord.gg/XgGxMSkvUM", class: FOOTER_LINK_CLASS, "Discord" }
              }
          }
          p {
              class: "mt-6 text-center text-xs text-[#5d5d5d]",
              "© 2026 Notes. Built with Dioxus."
          }
      }
  }
}
