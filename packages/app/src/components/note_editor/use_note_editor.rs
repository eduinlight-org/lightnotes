use crate::state::{use_notes, Note, NotesStore};
use dioxus::prelude::*;
use editor::{use_markdown_editor, MarkdownEditorHandle};

#[derive(Clone, Copy)]
pub struct NoteEditorState {
  pub store: NotesStore,
  pub markdown_editor: MarkdownEditorHandle,
  pub tag_draft: Signal<String>,
  pub tag_picker_open: Signal<bool>,
  pub folder_picker_open: Signal<bool>,
  synced_markdown: Signal<String>,
  last_note_id: Signal<Option<String>>,
}

impl NoteEditorState {
  pub fn sync_note(&mut self, note: &Note) {
    let note_switched = self.last_note_id.peek().as_deref() != Some(note.id.as_str());
    let external_change = !note_switched && note.content != *self.synced_markdown.peek();
    if note_switched || external_change {
      let normalized = self.markdown_editor.load(&note.content);
      self.synced_markdown.set(normalized.clone());
      self.last_note_id.set(Some(note.id.clone()));
      if normalized != note.content {
        self.store.set_note_content(&note.id, normalized);
      }
    }
  }

  pub fn save_if_changed(&mut self, note_id: &str) {
    let current = self.markdown_editor.markdown();
    if current != *self.synced_markdown.peek() {
      self.synced_markdown.set(current.clone());
      self.store.set_note_content(note_id, current);
    }
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
    markdown_editor: use_markdown_editor(""),
    tag_draft: use_signal(String::new),
    tag_picker_open: use_signal(|| false),
    folder_picker_open: use_signal(|| false),
    synced_markdown: use_signal(String::new),
    last_note_id: use_signal(|| None),
  }
}
