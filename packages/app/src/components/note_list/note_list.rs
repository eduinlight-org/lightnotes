use crate::components::NoteListItem;
use crate::state::use_notes;
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Plus, Search};
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::input::Input;
use ui::components::sidebar::SidebarTrigger;

#[component]
pub fn NoteList() -> Element {
  let mut store = use_notes();
  let route = use_route::<Route>();
  let active_note_id = match route {
    Route::NoteEditor { note_id } => Some(note_id),
    _ => None,
  };

  let notes = store.visible_notes();

  rsx! {
      div { class: "flex h-full w-full flex-col border-r border-white/10 md:w-80",
          div { class: "flex items-center gap-2 border-b border-white/10 p-3",
              SidebarTrigger {}
              div { class: "relative flex-1",
                  Search {
                      class: "pointer-events-none absolute left-2 top-1/2 -translate-y-1/2 text-[#5d5d5d]",
                      size: "16px",
                  }
                  Input {
                      class: "pl-8",
                      placeholder: "Search notes...",
                      value: store.search(),
                      oninput: move |event: FormEvent| store.set_search(event.value()),
                  }
              }
              Button {
                  variant: ButtonVariant::Primary,
                  size: ButtonSize::IconSm,
                  "aria-label": "New note",
                  onclick: move |_| {
                      let note_id = store.create_note();
                      navigator().push(Route::NoteEditor { note_id });
                  },
                  Plus { size: "16px" }
              }
          }
          div { class: "flex-1 overflow-y-auto p-2",
              if notes.is_empty() {
                  p { class: "px-2 py-6 text-center text-sm text-[#5d5d5d]", "No notes match." }
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
