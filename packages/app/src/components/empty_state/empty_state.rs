use dioxus::prelude::*;
use dioxus_icons::lucide::NotebookPen;
use ui::components::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn EmptyState() -> Element {
  rsx! {
      section {
          class: "flex flex-col items-center gap-4 px-6 py-24 text-center",
          NotebookPen { size: "48px", stroke: "#f59e0b" }
          h1 {
              class: "text-2xl font-bold text-white",
              "No notes yet"
          }
          p {
              class: "max-w-sm text-[#a1a1a1]",
              "Create your first text note or todo list to get started."
          }
          Button { variant: ButtonVariant::Primary, size: ButtonSize::Lg, "New note" }
      }
  }
}
