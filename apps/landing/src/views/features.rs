use dioxus::prelude::*;
use dioxus_icons::lucide::{FileText, SquareCheck};
use ui::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn Features() -> Element {
  rsx! {
      section {
          id: "features",
          class: "mx-auto grid max-w-5xl grid-cols-1 gap-6 px-6 py-24 sm:grid-cols-2",
          Card {
              CardHeader {
                  FileText { size: "28px", stroke: "#f59e0b" }
                  CardTitle { "Text notes" }
              }
              CardContent {
                  CardDescription { "Write a title and freeform markdown content for anything that needs more than a checkbox." }
                  ul {
                      class: "mt-4 list-inside list-disc text-[#a1a1a1]",
                      li { "Markdown formatting as you type" }
                      li { "Tag notes to group related ideas" }
                      li { "Search the full content of every note" }
                  }
              }
          }
          Card {
              CardHeader {
                  SquareCheck { size: "28px", stroke: "#ef4444" }
                  CardTitle { "Todo notes" }
              }
              CardContent {
                  CardDescription { "Give a note a title and a list of tasks, then check them off as you go." }
                  ul {
                      class: "mt-4 list-inside list-disc text-[#a1a1a1]",
                      li { "Add, reorder and check off items" }
                      li { "Track progress at a glance" }
                      li { "Same search and tagging as text notes" }
                  }
              }
          }
      }
  }
}
