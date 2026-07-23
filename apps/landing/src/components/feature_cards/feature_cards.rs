use dioxus::prelude::*;
use dioxus_icons::lucide::{FileText, Search, SquareCheck};
use ui::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn FeatureCards() -> Element {
  rsx! {
      section {
          id: "features-preview",
          class: "mx-auto grid max-w-5xl grid-cols-1 gap-6 px-6 py-16 sm:grid-cols-3",
          Card {
              CardHeader {
                  FileText { size: "28px", stroke: "#f59e0b" }
                  CardTitle { "Text notes" }
              }
              CardContent {
                  CardDescription { "Write freeform notes with markdown formatting for the ideas that need more than a checkbox." }
              }
          }
          Card {
              CardHeader {
                  SquareCheck { size: "28px", stroke: "#ef4444" }
                  CardTitle { "Todo notes" }
              }
              CardContent {
                  CardDescription { "Turn a note into a checklist and track what's done, one item at a time." }
              }
          }
          Card {
              CardHeader {
                  Search { size: "28px", stroke: "#2b7fff" }
                  CardTitle { "Find anything fast" }
              }
              CardContent {
                  CardDescription { "Search across every text note and todo list to find what you need in seconds." }
              }
          }
      }
  }
}
