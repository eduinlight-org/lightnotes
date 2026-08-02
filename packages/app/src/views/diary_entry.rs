use crate::components::{
  DateTimeFields, EmptyState, NoteEditorPanel, NoteEditorSkeleton, ReminderPicker,
};
use crate::state::{use_boot, use_notes};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn DiaryEntry(note_id: String) -> Element {
  let store = use_notes();
  let ready = (use_boot().store_ready)();

  if !ready {
    return rsx! {
        NoteEditorSkeleton {}
    };
  }

  match store.note(&note_id) {
    Some(note) => rsx! {
        NoteEditorPanel {
            note: note.clone(),
            extra_header: rsx! {
                DateTimeFields { note: note.clone() }
                ReminderPicker { note }
            },
            back_route: Route::Diary {},
        }
    },
    None => rsx! {
        EmptyState {}
    },
  }
}
