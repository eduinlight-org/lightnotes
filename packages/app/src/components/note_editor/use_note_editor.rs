use crate::state::{use_notes, Note, NotesStore};
use dioxus::prelude::*;
use editor::{use_markdown_editor, MarkdownEditorHandle};

#[derive(Clone, Copy, PartialEq)]
pub struct NoteEditorState {
  pub store: NotesStore,
  pub markdown_editor: MarkdownEditorHandle,
  pub tag_draft: Signal<String>,
  pub tag_picker_open: Signal<bool>,
  pub folder_picker_open: Signal<bool>,
  pub link_dialog_open: Signal<bool>,
  pub link_text_draft: Signal<String>,
  pub link_url_draft: Signal<String>,
  synced_markdown: Signal<String>,
  last_seen_content: Signal<String>,
  last_note_id: Signal<Option<String>>,
}

impl NoteEditorState {
  pub fn sync_note(&mut self, note: &Note) {
    let note_switched = self.last_note_id.peek().as_deref() != Some(note.id.as_str());
    let external_change = !note_switched && note.content != *self.last_seen_content.peek();
    if note_switched || external_change {
      let normalized = self.markdown_editor.load(&note.content);
      self.synced_markdown.set(normalized);
      self.last_seen_content.set(note.content.clone());
      self.last_note_id.set(Some(note.id.clone()));
    }
  }

  pub fn save_if_changed(&mut self, note_id: &str) {
    let current = self.markdown_editor.markdown();
    if current != *self.synced_markdown.peek() {
      self.synced_markdown.set(current.clone());
      self.last_seen_content.set(current.clone());
      self.store.set_note_content(note_id, current);
    }
  }

  pub fn open_link_dialog(&mut self) {
    let mut link_text_draft = self.link_text_draft;
    let mut link_url_draft = self.link_url_draft;
    let mut link_dialog_open = self.link_dialog_open;
    dioxus::prelude::spawn(async move {
      if let Some((text, href)) = editor::current_link().await {
        link_text_draft.set(text);
        link_url_draft.set(href);
      } else {
        link_text_draft.set(editor::selected_text().await);
        link_url_draft.set(String::new());
      }
      link_dialog_open.set(true);
    });
  }

  pub fn submit_link_dialog(&mut self) {
    let text = (self.link_text_draft)().trim().to_string();
    let href = (self.link_url_draft)().trim().to_string();
    if text.is_empty() || href.is_empty() {
      return;
    }
    self.markdown_editor.insert_link(text, href);
    self.link_dialog_open.set(false);
  }

  pub fn close_link_dialog(&mut self) {
    self.link_dialog_open.set(false);
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
    link_dialog_open: use_signal(|| false),
    link_text_draft: use_signal(String::new),
    link_url_draft: use_signal(String::new),
    synced_markdown: use_signal(String::new),
    last_seen_content: use_signal(String::new),
    last_note_id: use_signal(|| None),
  }
}
