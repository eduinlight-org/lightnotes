use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::NotebookPen;
use ui::components::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn Hero() -> Element {
  rsx! {
      section {
          id: "hero",
          class: "flex flex-col items-center gap-6 px-6 py-24 text-center",
          div {
              class: "inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-1 text-sm text-[#d4d4d4]",
              NotebookPen { size: "16px", stroke: "#f59e0b" }
              "The simplest way to capture your thoughts"
          }
          h1 {
              class: "max-w-2xl text-4xl font-bold text-white sm:text-5xl",
              "All your notes, beautifully organized"
          }
          p {
              class: "max-w-xl text-lg text-[#a1a1a1]",
              "Write markdown text notes for your ideas, or turn a note into a todo list to track what's done. One fast, distraction-free app for both."
          }
          div {
              class: "flex flex-wrap items-center justify-center gap-4",
              Button { variant: ButtonVariant::Primary, size: ButtonSize::Lg, "Get started" }
              Link {
                  to: Route::Features {},
                  Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "See features" }
              }
          }
      }
  }
}
