use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

const LINK_CLASS: &str = "text-white no-underline my-2.5 mx-0 border border-white rounded-[5px] p-2.5 hover:bg-[#1f1f1f] hover:cursor-pointer";

#[component]
pub fn Hero() -> Element {
  rsx! {
      div {
          id: "hero",
          class: "m-0 flex flex-col justify-center items-center",
          img { src: HEADER_SVG, id: "header", class: "max-w-[1200px]" }
          div {
              id: "links",
              class: "w-[400px] text-left text-xl text-white flex flex-col",
              a { href: "https://dioxuslabs.com/learn/0.7/", class: LINK_CLASS, "📚 Learn Dioxus" }
              a { href: "https://dioxuslabs.com/awesome", class: LINK_CLASS, "🚀 Awesome Dioxus" }
              a { href: "https://github.com/dioxus-community/", class: LINK_CLASS, "📡 Community Libraries" }
              a { href: "https://github.com/DioxusLabs/sdk", class: LINK_CLASS, "⚙️ Dioxus Development Kit" }
              a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", class: LINK_CLASS, "💫 VSCode Extension" }
              a { href: "https://discord.gg/XgGxMSkvUM", class: LINK_CLASS, "👋 Community Discord" }
          }
      }
  }
}
