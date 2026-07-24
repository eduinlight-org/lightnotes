use crate::components::{AppSidebar, NoteList};
use crate::Route;
use dioxus::prelude::*;
use ui::components::sidebar::{SidebarInset, SidebarProvider};

#[component]
pub fn NotesShell() -> Element {
  let route = use_route::<Route>();
  let note_open = matches!(route, Route::NoteEditor { .. });

  rsx! {
      SidebarProvider { class: "h-full",
          AppSidebar {}
          SidebarInset {
              div { class: "flex h-full w-full overflow-hidden",
                  div {
                      class: if note_open { "hidden h-full md:flex" } else { "flex h-full" },
                      NoteList {}
                  }
                  div {
                      class: if note_open { "flex h-full flex-1" } else { "hidden h-full flex-1 md:flex" },
                      Outlet::<Route> {}
                  }
              }
          }
      }
  }
}
