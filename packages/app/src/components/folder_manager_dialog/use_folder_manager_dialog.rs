use crate::state::{use_notes, use_ui, FolderIcon, NoteFilter, NotesStore, UiState};
use crate::Route;
use dioxus::prelude::*;
use ui::components::sidebar::use_is_mobile;

#[derive(Clone, Copy)]
pub struct FolderManagerPanelState {
  pub store: NotesStore,
  pub draft: Signal<String>,
  pub draft_icon: Signal<FolderIcon>,
  pub is_mobile: Signal<bool>,
  pub on_select: Option<EventHandler<()>>,
  pub pending_delete: Signal<Option<String>>,
}

impl FolderManagerPanelState {
  pub fn submit(&mut self) {
    let name = (self.draft)();
    if name.trim().is_empty() {
      return;
    }
    self.store.create_folder_with_icon(name, (self.draft_icon)());
    self.draft.set(String::new());
  }

  pub fn select_folder(&mut self, folder_id: String) {
    self.store.set_filter(NoteFilter::Folder(folder_id));
    navigator().push(Route::Notes {});
    if let Some(handler) = &self.on_select {
      handler.call(());
    }
  }

  pub fn request_delete(&mut self, folder_id: &str) {
    self.pending_delete.set(Some(folder_id.to_string()));
  }

  pub fn cancel_delete(&mut self) {
    self.pending_delete.set(None);
  }

  pub fn confirm_delete(&mut self, folder_id: &str) {
    self.store.delete_folder(folder_id);
    self.pending_delete.set(None);
  }
}

pub fn use_folder_manager_panel(on_select: Option<EventHandler<()>>) -> FolderManagerPanelState {
  FolderManagerPanelState {
    store: use_notes(),
    draft: use_signal(String::new),
    draft_icon: use_signal(|| FolderIcon::Notebook),
    is_mobile: use_is_mobile(),
    on_select,
    pending_delete: use_signal(|| None),
  }
}

#[derive(Clone, Copy)]
pub struct FolderManagerDialogState {
  pub ui: UiState,
}

impl FolderManagerDialogState {
  pub fn open(&self) -> bool {
    (self.ui.folders_open)()
  }

  pub fn set_open(&mut self, value: bool) {
    self.ui.folders_open.set(value);
  }

  pub fn close(&mut self) {
    self.set_open(false);
  }
}

pub fn use_folder_manager_dialog() -> FolderManagerDialogState {
  FolderManagerDialogState { ui: use_ui() }
}
