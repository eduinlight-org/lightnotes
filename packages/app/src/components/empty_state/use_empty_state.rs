use crate::state::{use_notes, NotesStore};
use crate::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct EmptyStateState {
  pub store: NotesStore,
}

impl EmptyStateState {
  pub fn create_note(&mut self) {
    let note_id = self.store.create_note();
    navigator().push(Route::NoteEditor { note_id });
  }
}

pub fn use_empty_state() -> EmptyStateState {
  EmptyStateState { store: use_notes() }
}
