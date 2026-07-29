use crate::state::DiaryUiState;
use crate::Route;
use dioxus::prelude::*;
use ui::components::sidebar::use_is_mobile;

#[derive(Clone, Copy)]
pub struct DiaryShellState {
  pub is_mobile: Signal<bool>,
  pub full_page: bool,
}

pub fn use_diary_shell() -> DiaryShellState {
  use_context_provider(DiaryUiState::seed);
  let is_mobile = use_is_mobile();
  let route = use_route::<Route>();
  let full_page = matches!(route, Route::DiaryEntry { .. });

  DiaryShellState { is_mobile, full_page }
}
