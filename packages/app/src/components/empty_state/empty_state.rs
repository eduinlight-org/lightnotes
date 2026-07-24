use crate::state::use_notes;
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::NotebookPen;
use ui::components::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn EmptyState() -> Element {
  let mut store = use_notes();

  rsx! {
      section {
          class: "flex h-full flex-col items-center justify-center gap-4 px-6 py-24 text-center",
          NotebookPen { size: "48px", stroke: "#f59e0b" }
          h1 {
              class: "text-2xl font-bold text-white",
              "No note selected"
          }
          p {
              class: "max-w-sm text-[#a1a1a1]",
              "Choose a note from the list, or create a new one to get started."
          }
          Button {
              variant: ButtonVariant::Primary,
              size: ButtonSize::Lg,
              onclick: move |_| {
                  let note_id = store.create_note();
                  navigator().push(Route::NoteEditor { note_id });
              },
              "New note"
          }
      }
  }
}
