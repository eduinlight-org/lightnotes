use crate::state::{use_notes, NotesStore};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct FolderPickerState {
  pub store: NotesStore,
  pub open: Signal<bool>,
}

impl FolderPickerState {
  pub fn move_to_folder(&mut self, note_id: &str, folder_id: Option<String>) {
    self.store.set_note_folder(note_id, folder_id);
    self.open.set(false);
  }
}

pub fn use_folder_picker() -> FolderPickerState {
  FolderPickerState { store: use_notes(), open: use_signal(|| false) }
}
