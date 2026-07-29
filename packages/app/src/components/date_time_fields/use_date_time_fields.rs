use crate::state::{date_math, use_notes, NotesStore};

#[derive(Clone, Copy)]
pub struct DateTimeFieldsState {
  pub store: NotesStore,
}

impl DateTimeFieldsState {
  pub fn set_date(&mut self, note_id: &str, current_date_ms: i64, date_str: String) {
    let time_str = date_math::date_ms_to_time_string(current_date_ms);
    if let Some(new_date_ms) = date_math::date_and_time_strings_to_ms(&date_str, &time_str) {
      self.store.set_note_date(note_id, new_date_ms);
    }
  }

  pub fn set_time(&mut self, note_id: &str, current_date_ms: i64, time_str: String) {
    let date_str = date_math::date_ms_to_date_string(current_date_ms);
    if let Some(new_date_ms) = date_math::date_and_time_strings_to_ms(&date_str, &time_str) {
      self.store.set_note_date(note_id, new_date_ms);
    }
  }
}

pub fn use_date_time_fields() -> DateTimeFieldsState {
  DateTimeFieldsState { store: use_notes() }
}
