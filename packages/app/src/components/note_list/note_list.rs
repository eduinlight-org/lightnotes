use super::use_note_list::use_note_list;
use crate::components::{NoteListItem, SearchInput};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Inbox, Plus, Search};
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::sidebar::SidebarTrigger;

#[component]
pub fn NoteList() -> Element {
  let (mut note_list, active_note_id) = use_note_list();
  let mut store = note_list.store;

  let search = store.search();
  let has_search = !search.trim().is_empty();
  let notes = store.visible_notes();

  let (title, subtitle) = if has_search {
    (
      "Search".to_string(),
      format!("{} result{} for \u{201c}{}\u{201d}", notes.len(), if notes.len() == 1 { "" } else { "s" }, search.trim()),
    )
  } else {
    (store.filter_title(), format!("{} notes", notes.len()))
  };

  rsx! {
      div { class: "flex h-full w-full flex-col border-r border-[var(--primary-color-6)] md:w-80",
          div { class: "flex flex-none flex-col gap-2 border-b border-[var(--primary-color-6)] p-3",
              div { class: "flex items-center gap-2",
                  SidebarTrigger {}
                  div { class: "min-w-0 flex-1",
                      div { class: "truncate text-base font-medium text-[var(--secondary-color)]", "{title}" }
                      div { class: "truncate text-xs text-[var(--secondary-color-5)]", "{subtitle}" }
                  }
                  Button {
                      variant: ButtonVariant::Secondary,
                      size: ButtonSize::IconSm,
                      class: "border border-[var(--primary-color-6)] bg-transparent hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]",
                      "aria-label": "New note",
                      onclick: move |_| note_list.create_note(),
                      Plus { size: "16px" }
                  }
              }
              SearchInput { value: search.clone(), on_search: move |value| store.set_search(value) }
          }
          div { class: "min-h-0 flex-1 overflow-y-auto p-2",
              if notes.is_empty() {
                  div { class: "flex h-full flex-col items-center justify-center gap-2 px-6 py-16 text-center",
                      if has_search {
                          Search { size: "36px", class: "opacity-40" }
                      } else {
                          Inbox { size: "36px", class: "opacity-40" }
                      }
                      div { class: "text-sm font-medium text-[var(--secondary-color)]",
                          if has_search { "No matches" } else { "No notes yet" }
                      }
                      p { class: "max-w-[200px] text-xs text-[var(--secondary-color-5)]",
                          if has_search { "Try a different search term." } else { "Create your first note in this view." }
                      }
                  }
              }
              for note in notes {
                  {
                      let is_active = active_note_id.as_deref() == Some(note.id.as_str());
                      rsx! {
                          NoteListItem { key: "{note.id}", note, is_active }
                      }
                  }
              }
          }
      }
  }
}
