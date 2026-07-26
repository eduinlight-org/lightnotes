use crate::state::{use_notes, Note, NotesStore};
use crate::Route;
use dioxus::prelude::*;
use ui::components::sidebar::use_is_mobile;

#[derive(Clone, Copy)]
pub struct NoteListItemState {
  pub store: NotesStore,
  pub is_mobile: Signal<bool>,
}

impl NoteListItemState {
  pub fn open(&self, note_id: &str) {
    navigator().push(Route::NoteEditor { note_id: note_id.to_string() });
  }

  pub fn toggle_star(&mut self, note_id: &str) {
    self.store.toggle_note_star(note_id);
  }

  pub fn toggle_pin(&mut self, note_id: &str) {
    self.store.toggle_note_pin(note_id);
  }

  pub fn tags(&self, note: &Note) -> Vec<String> {
    note.tag_ids.iter().filter_map(|id| self.store.tag_name(id)).take(3).collect()
  }
}

pub fn use_note_list_item() -> NoteListItemState {
  NoteListItemState { store: use_notes(), is_mobile: use_is_mobile() }
}
