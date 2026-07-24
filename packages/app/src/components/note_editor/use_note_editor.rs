use crate::state::{use_notes, NotesStore};
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorView {
  Edit,
  Preview,
  Split,
}

#[derive(Clone, Copy)]
pub struct NoteEditorState {
  pub store: NotesStore,
  pub view: Signal<EditorView>,
  pub tag_draft: Signal<String>,
}

impl NoteEditorState {
  pub fn append_snippet(&mut self, note_id: &str, current_content: &str, snippet: &str) {
    self
      .store
      .set_note_content(note_id, format!("{current_content}{snippet}"));
  }

  pub fn commit_tag_draft(&mut self, note_id: &str) {
    let name = (self.tag_draft)().trim().to_string();
    if name.is_empty() {
      return;
    }

    let tag_id = self.store.tag_id_for_name(&name);
    self.store.add_note_tag(note_id, tag_id);
    self.tag_draft.set(String::new());
  }
}

pub fn use_note_editor() -> NoteEditorState {
  NoteEditorState {
    store: use_notes(),
    view: use_signal(|| EditorView::Edit),
    tag_draft: use_signal(String::new),
  }
}
