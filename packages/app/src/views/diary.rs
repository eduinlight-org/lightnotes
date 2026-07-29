use crate::state::{use_diary_ui, use_notes};
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Feather, Plus};
use ui::components::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn Diary() -> Element {
  let mut store = use_notes();
  let diary_ui = use_diary_ui();

  rsx! {
      section { class: "flex h-full w-full flex-col items-center justify-center gap-4 px-6 py-24 text-center",
          Feather { size: "48px", stroke: "var(--accent)" }
          h1 { class: "text-2xl font-medium text-[var(--secondary-color)]", "No note selected" }
          p { class: "max-w-sm text-[var(--secondary-color-5)]", "Pick a day on the calendar, or write something new." }
          Button {
              variant: ButtonVariant::Primary,
              size: ButtonSize::Lg,
              class: "gap-2 border border-[var(--accent)] bg-transparent text-[var(--accent)] hover:bg-[color-mix(in_srgb,var(--accent)_12%,transparent)]",
              onclick: move |_| {
                  let folder_id = (diary_ui.filter_folder)();
                  let tag_ids = (diary_ui.filter_tag)().into_iter().collect();
                  let note_id = store.create_diary_note((diary_ui.cursor_date_ms)(), folder_id, tag_ids);
                  navigator().push(Route::DiaryEntry { note_id });
              },
              Plus { size: "16px" }
              "New note"
          }
      }
  }
}
