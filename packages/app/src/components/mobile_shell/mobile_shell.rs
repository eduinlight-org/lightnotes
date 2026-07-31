use super::use_mobile_shell::use_mobile_shell;
use crate::components::{NoteListItem, SearchInput, SearchInputSize};
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_icons::lucide::{CirclePlus, Inbox, Search};
use ui::components::sidebar::SidebarTrigger;

#[component]
pub fn MobileShell() -> Element {
  let mut mobile_shell = use_mobile_shell();

  if mobile_shell.full_page {
    return rsx! {
        div { class: "flex h-full flex-col", Outlet::<Route> {} }
    };
  }

  let mut store = mobile_shell.store;
  let search = store.search();
  let has_search = !search.trim().is_empty();
  let notes = store.visible_notes();
  let title = store.filter_title();

  rsx! {
      div { class: "relative flex h-full min-h-0 flex-col overflow-hidden",
          div { class: "flex flex-none items-center gap-3 p-3",
              SidebarTrigger { label: t!("sidebar-toggle") }
              div { class: "min-w-0 flex-1 truncate text-lg font-medium text-[var(--secondary-color)]", "{title}" }
              button {
                  "aria-label": t!("action-new-note"),
                  onclick: move |_| mobile_shell.create_note(),
                  CirclePlus { size: "22px", stroke: "var(--accent)" }
              }
          }
          div { class: "flex-none px-3 pb-3",
              SearchInput {
                  value: search.clone(),
                  on_search: move |value| store.set_search(value),
                  size: SearchInputSize::Large,
              }
          }
          div { class: "min-h-0 flex-1 overflow-y-auto px-3 pb-4",
              if notes.is_empty() {
                  div { class: "flex flex-col items-center gap-2 py-16 text-center text-[var(--secondary-color-5)]",
                      if has_search {
                          Search { size: "40px", class: "opacity-40" }
                      } else {
                          Inbox { size: "40px", class: "opacity-40" }
                      }
                      div { class: "text-base font-medium text-[var(--secondary-color)]",
                          if has_search { {t!("notes-empty-no-matches")} } else { {t!("notes-empty-no-notes")} }
                      }
                      p { class: "text-sm",
                          if has_search { {t!("notes-empty-search-hint")} } else { {t!("notes-empty-hint")} }
                      }
                  }
              }
              for note in notes {
                  NoteListItem { key: "{note.id}", note, is_active: false }
              }
          }
      }
  }
}
