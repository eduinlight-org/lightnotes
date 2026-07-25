use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct UiState {
  pub tags_open: Signal<bool>,
  pub folders_open: Signal<bool>,
}

impl UiState {
  pub fn seed() -> Self {
    Self { tags_open: Signal::new(false), folders_open: Signal::new(false) }
  }
}

pub fn use_ui() -> UiState {
  use_context::<UiState>()
}
