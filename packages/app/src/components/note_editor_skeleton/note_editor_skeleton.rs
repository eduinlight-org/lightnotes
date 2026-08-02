use dioxus::prelude::*;
use ui::components::skeleton::Skeleton;

fn line_width(index: usize) -> &'static str {
  match index % 5 {
    0 => "w-11/12",
    1 => "w-full",
    2 => "w-9/12",
    3 => "w-full",
    _ => "w-7/12",
  }
}

#[component]
pub fn NoteEditorSkeleton() -> Element {
  rsx! {
      div { class: "flex h-full w-full flex-col overflow-hidden", "aria-busy": "true",
          div { class: "flex flex-none items-center gap-3 border-b border-[var(--primary-color-6)] p-3",
              Skeleton { class: "h-7 w-7 flex-none rounded-md" }
              Skeleton { class: "h-5 flex-1 max-w-[280px]" }
              Skeleton { class: "h-7 w-7 flex-none rounded-md" }
              Skeleton { class: "h-7 w-7 flex-none rounded-md" }
          }
          div { class: "flex flex-none items-center gap-2 px-4 py-3",
              Skeleton { class: "h-6 w-24 rounded-full" }
              Skeleton { class: "h-6 w-16 rounded-full" }
              Skeleton { class: "h-6 w-20 rounded-full" }
          }
          div { class: "flex min-h-0 flex-1 flex-col gap-3 px-4 pb-6",
              for index in 0..10 {
                  Skeleton { key: "{index}", class: "h-3.5 {line_width(index)}" }
              }
          }
      }
  }
}
