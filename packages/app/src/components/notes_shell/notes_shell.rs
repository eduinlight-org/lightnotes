use crate::components::{AppSidebar, FolderManagerDialog, MobileShell, NoteList, TagManagerDialog};
use crate::state::UiState;
use crate::Route;
use dioxus::prelude::*;
use ui::components::sidebar::{use_is_mobile, SidebarInset, SidebarProvider};

#[component]
pub fn NotesShell() -> Element {
  use_context_provider(UiState::seed);
  let is_mobile = use_is_mobile();
  let route = use_route::<Route>();
  let full_page = matches!(route, Route::NoteEditor { .. } | Route::TagsScreen { .. } | Route::FoldersScreen { .. });

  rsx! {
      TagManagerDialog {}
      FolderManagerDialog {}
      SidebarProvider { class: "h-full! min-h-0!",
          AppSidebar {}
          SidebarInset {
              if is_mobile() {
                  MobileShell {}
              } else {
                  div { class: "flex h-full w-full overflow-hidden",
                      div {
                          class: if full_page { "hidden h-full md:flex" } else { "flex h-full" },
                          NoteList {}
                      }
                      div {
                          class: if full_page { "flex h-full flex-1" } else { "hidden h-full flex-1 md:flex" },
                          Outlet::<Route> {}
                      }
                  }
              }
          }
      }
  }
}
