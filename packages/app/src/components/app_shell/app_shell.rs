use crate::components::{AppBar, LoginButton};
use crate::state::NotesStore;
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::NotebookPen;

#[component]
pub fn AppShell() -> Element {
  use_context_provider(NotesStore::seed);

  rsx! {
      div { class: "flex h-screen flex-col bg-[#0f1116]",
          AppBar {
              brand: rsx! {
                  Link {
                      to: Route::Notes {},
                      class: "flex items-center gap-2 whitespace-nowrap text-white no-underline",
                      NotebookPen { size: "18px" }
                      span { class: "hidden sm:inline", "Notes" }
                  }
              },
              actions: rsx! { LoginButton {} },
          }
          div { class: "flex-1 overflow-hidden", Outlet::<Route> {} }
      }
  }
}
