use crate::state::{use_notes, use_ui, NoteFilter, NotesStore, UiState};
use crate::Route;
use dioxus::prelude::*;
use ui::components::sidebar::use_is_mobile;

#[derive(Clone, Copy)]
pub struct TagManagerPanelState {
  pub store: NotesStore,
  pub draft: Signal<String>,
  pub is_mobile: Signal<bool>,
  pub on_select: Option<EventHandler<()>>,
  pub pending_delete: Signal<Option<String>>,
}

impl TagManagerPanelState {
  pub fn submit(&mut self) {
    let name = (self.draft)();
    if name.trim().is_empty() {
      return;
    }
    self.store.create_tag(name);
    self.draft.set(String::new());
  }

  pub fn select_tag(&mut self, tag_id: String) {
    self.store.set_filter(NoteFilter::Tag(tag_id));
    navigator().push(Route::Notes {});
    if let Some(handler) = &self.on_select {
      handler.call(());
    }
  }

  pub fn request_delete(&mut self, tag_id: &str) {
    self.pending_delete.set(Some(tag_id.to_string()));
  }

  pub fn cancel_delete(&mut self) {
    self.pending_delete.set(None);
  }

  pub fn confirm_delete(&mut self, tag_id: &str) {
    self.store.delete_tag(tag_id);
    self.pending_delete.set(None);
  }
}

pub fn use_tag_manager_panel(on_select: Option<EventHandler<()>>) -> TagManagerPanelState {
  TagManagerPanelState {
    store: use_notes(),
    draft: use_signal(String::new),
    is_mobile: use_is_mobile(),
    on_select,
    pending_delete: use_signal(|| None),
  }
}

#[derive(Clone, Copy)]
pub struct TagManagerDialogState {
  pub ui: UiState,
}

impl TagManagerDialogState {
  pub fn open(&self) -> bool {
    (self.ui.tags_open)()
  }

  pub fn set_open(&mut self, value: bool) {
    self.ui.tags_open.set(value);
  }

  pub fn close(&mut self) {
    self.set_open(false);
  }
}

pub fn use_tag_manager_dialog() -> TagManagerDialogState {
  TagManagerDialogState { ui: use_ui() }
}
