use crate::state::{use_notes, Language, NotesStore};
use dioxus::prelude::*;
use ui::components::sidebar::use_is_mobile;

#[derive(Clone, Copy)]
pub struct LanguagePickerState {
  pub store: NotesStore,
  pub open: Signal<bool>,
  pub is_mobile: Signal<bool>,
}

impl LanguagePickerState {
  pub fn language(&self) -> Language {
    self.store.language()
  }

  pub fn set_open(&mut self, open: bool) {
    self.open.set(open);
  }

  pub fn select(&mut self, language: Language) {
    self.store.set_language(language);
    self.open.set(false);
  }
}

pub fn use_language_picker() -> LanguagePickerState {
  LanguagePickerState { store: use_notes(), open: use_signal(|| false), is_mobile: use_is_mobile() }
}
