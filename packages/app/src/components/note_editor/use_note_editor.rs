use crate::state::{use_notes, NotesStore};
use dioxus::prelude::*;

pub const EDITOR_TEXTAREA_ID: &str = "lightnotes-editor-textarea";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorView {
  Live,
  Source,
  Preview,
}

#[derive(Clone, Copy)]
pub struct NoteEditorState {
  pub store: NotesStore,
  pub view: Signal<EditorView>,
  pub tag_draft: Signal<String>,
  pub tag_picker_open: Signal<bool>,
  pub folder_picker_open: Signal<bool>,
}

async fn read_wrapped_selection(prefix: &'static str, suffix: &'static str) -> Option<(String, i64, i64)> {
  let mut eval = document::eval(
    r#"
    const [id, pre, post] = await dioxus.recv();
    const el = document.getElementById(id);
    if (!el) { dioxus.send(null); }
    else {
      const s = el.selectionStart, e = el.selectionEnd, v = el.value;
      const sel = v.slice(s, e) || 'text';
      const nv = v.slice(0, s) + pre + sel + post + v.slice(e);
      dioxus.send([nv, s + pre.length, s + pre.length + sel.length]);
    }
    "#,
  );
  let _ = eval.send((EDITOR_TEXTAREA_ID, prefix, suffix));
  eval.recv::<Option<(String, i64, i64)>>().await.ok().flatten()
}

async fn read_line_prefixed(prefix: &'static str) -> Option<(String, i64, i64)> {
  let mut eval = document::eval(
    r#"
    const [id, pre] = await dioxus.recv();
    const el = document.getElementById(id);
    if (!el) { dioxus.send(null); }
    else {
      const s = el.selectionStart, v = el.value;
      const ls = v.lastIndexOf('\n', s - 1) + 1;
      const nv = v.slice(0, ls) + pre + v.slice(ls);
      dioxus.send([nv, s + pre.length, s + pre.length]);
    }
    "#,
  );
  let _ = eval.send((EDITOR_TEXTAREA_ID, prefix));
  eval.recv::<Option<(String, i64, i64)>>().await.ok().flatten()
}

async fn restore_selection(start: i64, end: i64) {
  let eval = document::eval(
    r#"
    const [id, s, e] = await dioxus.recv();
    requestAnimationFrame(() => {
      const el = document.getElementById(id);
      if (el) { el.focus(); el.setSelectionRange(s, e); }
    });
    "#,
  );
  let _ = eval.send((EDITOR_TEXTAREA_ID, start, end));
}

impl NoteEditorState {
  pub fn wrap_selection(&mut self, note_id: &str, prefix: &'static str, suffix: &'static str) {
    let mut store = self.store;
    let note_id = note_id.to_string();
    spawn(async move {
      if let Some((value, start, end)) = read_wrapped_selection(prefix, suffix).await {
        store.set_note_content(&note_id, value);
        restore_selection(start, end).await;
      }
    });
  }

  pub fn line_prefix(&mut self, note_id: &str, prefix: &'static str) {
    let mut store = self.store;
    let note_id = note_id.to_string();
    spawn(async move {
      if let Some((value, start, end)) = read_line_prefixed(prefix).await {
        store.set_note_content(&note_id, value);
        restore_selection(start, end).await;
      }
    });
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
    view: use_signal(|| EditorView::Source),
    tag_draft: use_signal(String::new),
    tag_picker_open: use_signal(|| false),
    folder_picker_open: use_signal(|| false),
  }
}
