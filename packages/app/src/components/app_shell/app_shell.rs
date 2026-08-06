use crate::components::{AppBar, UserAvatar};
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::NotebookText;

#[component]
pub fn AppShell() -> Element {
  rsx! {
      div { class: "flex h-dvh flex-col bg-[var(--primary-color-1)]",
          AppBar {
              brand: rsx! {
                  Link {
                      to: Route::Notes {},
                      class: "flex items-center gap-2 whitespace-nowrap text-[var(--secondary-color)] no-underline",
                      NotebookText { size: "18px", stroke: "var(--accent)" }
                      span { "LightNotes" }
                  }
              },
              actions: rsx! {
                  UserAvatar {}
              },
          }
          div { class: "min-h-0 flex-1 overflow-hidden", Outlet::<Route> {} }
      }
  }
}
