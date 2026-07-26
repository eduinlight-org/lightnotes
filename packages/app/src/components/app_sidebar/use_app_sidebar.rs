use crate::state::{use_notes, use_ui, NoteFilter, NotesStore, UiState};
use crate::Route;
use dioxus::prelude::*;
use ui::components::sidebar::{use_is_mobile, use_sidebar, SidebarCtx};

#[derive(Clone, Copy)]
pub struct AppSidebarState {
  pub store: NotesStore,
  pub ui: UiState,
  pub sidebar_ctx: SidebarCtx,
  pub is_mobile: Signal<bool>,
}

impl AppSidebarState {
  pub fn select_filter(&mut self, filter: NoteFilter) {
    self.store.set_filter(filter);
    self.sidebar_ctx.set_open_mobile(false);
    navigator().push(Route::Notes {});
  }

  pub fn create_note(&mut self) {
    let note_id = self.store.create_note();
    self.sidebar_ctx.set_open_mobile(false);
    navigator().push(Route::NoteEditor { note_id });
  }

  pub fn open_folders_manager(&mut self) {
    self.sidebar_ctx.set_open_mobile(false);
    if (self.is_mobile)() {
      navigator().push(Route::FoldersScreen {});
    } else {
      self.ui.folders_open.set(true);
    }
  }

  pub fn open_tags_manager(&mut self) {
    self.sidebar_ctx.set_open_mobile(false);
    if (self.is_mobile)() {
      navigator().push(Route::TagsScreen {});
    } else {
      self.ui.tags_open.set(true);
    }
  }

  pub fn toggle_sync(&mut self) {
    self.store.toggle_sync();
  }
}

pub fn use_app_sidebar() -> AppSidebarState {
  AppSidebarState { store: use_notes(), ui: use_ui(), sidebar_ctx: use_sidebar(), is_mobile: use_is_mobile() }
}
