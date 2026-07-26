use crate::state::UiState;
use crate::Route;
use dioxus::prelude::*;
use ui::components::sidebar::use_is_mobile;

#[derive(Clone, Copy)]
pub struct NotesShellState {
  pub is_mobile: Signal<bool>,
  pub full_page: bool,
}

pub fn use_notes_shell() -> NotesShellState {
  use_context_provider(UiState::seed);
  let is_mobile = use_is_mobile();
  let route = use_route::<Route>();
  let full_page = matches!(route, Route::NoteEditor { .. } | Route::TagsScreen { .. } | Route::FoldersScreen { .. });

  NotesShellState { is_mobile, full_page }
}
