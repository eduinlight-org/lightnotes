use crate::state::{use_notes, NotesStore};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct ReminderPickerState {
  pub store: NotesStore,
  pub open: Signal<bool>,
}

impl ReminderPickerState {
  pub fn set_remind_before(&mut self, note_id: &str, hours: Option<i64>) {
    self.store.set_note_remind_before(note_id, hours);
    self.open.set(false);
  }
}

pub fn use_reminder_picker() -> ReminderPickerState {
  ReminderPickerState { store: use_notes(), open: use_signal(|| false) }
}
