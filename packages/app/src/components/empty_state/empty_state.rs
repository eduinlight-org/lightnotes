use super::use_empty_state::use_empty_state;
use dioxus::prelude::*;
use dioxus_icons::lucide::{NotebookPen, Plus};
use ui::components::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn EmptyState() -> Element {
  let mut empty_state = use_empty_state();

  rsx! {
      section {
          class: "flex h-full w-full flex-col items-center justify-center gap-4 px-6 py-24 text-center",
          NotebookPen { size: "48px", stroke: "var(--accent)" }
          h1 {
              class: "text-2xl font-medium text-[var(--secondary-color)]",
              "No note selected"
          }
          p {
              class: "max-w-sm text-[var(--secondary-color-5)]",
              "Choose a note from the list, or create a new one to get started."
          }
          Button {
              variant: ButtonVariant::Primary,
              size: ButtonSize::Lg,
              class: "gap-2 border border-[var(--accent)] bg-transparent text-[var(--accent)] hover:bg-[color-mix(in_srgb,var(--accent)_12%,transparent)]",
              onclick: move |_| empty_state.create_note(),
              Plus { size: "16px" }
              "New note"
          }
      }
  }
}
