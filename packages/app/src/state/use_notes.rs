use crate::state::notes::NotesStore;
use dioxus::prelude::*;

pub fn use_notes() -> NotesStore {
  use_context::<NotesStore>()
}
