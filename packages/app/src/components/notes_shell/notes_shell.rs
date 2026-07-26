use super::use_notes_shell::use_notes_shell;
use crate::components::{AppSidebar, FolderManagerDialog, MobileShell, NoteList, TagManagerDialog};
use crate::Route;
use dioxus::prelude::*;
use ui::components::sidebar::{SidebarInset, SidebarProvider};

#[component]
pub fn NotesShell() -> Element {
  let notes_shell = use_notes_shell();
  let is_mobile = notes_shell.is_mobile;
  let full_page = notes_shell.full_page;

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
