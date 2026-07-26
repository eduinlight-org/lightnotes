use crate::state::{use_notes, NotesStore};
use crate::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct MobileShellState {
  pub store: NotesStore,
  pub full_page: bool,
}

impl MobileShellState {
  pub fn create_note(&mut self) {
    let note_id = self.store.create_note();
    navigator().push(Route::NoteEditor { note_id });
  }
}

pub fn use_mobile_shell() -> MobileShellState {
  let store = use_notes();
  let route = use_route::<Route>();
  let full_page = matches!(route, Route::NoteEditor { .. } | Route::TagsScreen { .. } | Route::FoldersScreen { .. });

  MobileShellState { store, full_page }
}
