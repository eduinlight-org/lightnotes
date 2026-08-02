use crate::state::{use_boot, use_notes, NotesStore};
use crate::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct NoteListState {
  pub store: NotesStore,
  pub ready: bool,
}

impl NoteListState {
  pub fn create_note(&mut self) {
    let note_id = self.store.create_note();
    navigator().push(Route::NoteEditor { note_id });
  }
}

pub fn use_note_list() -> (NoteListState, Option<String>) {
  let store = use_notes();
  let ready = (use_boot().store_ready)();
  let route = use_route::<Route>();
  let active_note_id = match route {
    Route::NoteEditor { note_id } => Some(note_id),
    _ => None,
  };

  (NoteListState { store, ready }, active_note_id)
}
