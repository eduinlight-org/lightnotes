use crate::components::TagManagerPanel;
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::ArrowLeft;

#[component]
pub fn TagsScreen() -> Element {
  rsx! {
      div { class: "flex h-full flex-col overflow-hidden",
          div { class: "flex flex-none items-center gap-3 border-b border-[var(--primary-color-6)] p-3",
              button {
                  class: "flex items-center gap-1 text-sm font-medium text-[var(--accent)]",
                  onclick: move |_| {
                      navigator().push(Route::Notes {});
                  },
                  ArrowLeft { size: "18px" }
                  "Notes"
              }
              h1 { class: "flex-1 text-lg font-medium text-[var(--secondary-color)]", "Tags" }
          }
          div { class: "flex min-h-0 flex-1 flex-col overflow-hidden p-4",
              TagManagerPanel {
                  on_select: move |_| {
                      navigator().push(Route::Notes {});
                  },
              }
          }
      }
  }
}
