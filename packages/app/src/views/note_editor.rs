use crate::components::{EmptyState, NoteEditorPanel};
use crate::state::use_notes;
use dioxus::prelude::*;

#[component]
pub fn NoteEditor(note_id: String) -> Element {
  let store = use_notes();

  match store.note(&note_id) {
    Some(note) => rsx! {
        NoteEditorPanel { note }
    },
    None => rsx! {
        EmptyState {}
    },
  }
}
