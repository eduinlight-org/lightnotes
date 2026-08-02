use dioxus::prelude::*;
use ui::components::skeleton::Skeleton;

fn rows_in_section(index: usize) -> usize {
  match index {
    0 => 2,
    3 => 3,
    _ => 1,
  }
}

#[component]
pub fn SettingsSkeleton() -> Element {
  rsx! {
      div { class: "flex flex-col gap-5", "aria-busy": "true",
          for section in 0..5 {
              div { key: "{section}", class: "flex flex-col gap-2",
                  Skeleton { class: "mb-1 h-3 w-24" }
                  for row in 0..rows_in_section(section) {
                      div { key: "{row}", class: "flex items-center gap-4 rounded-lg bg-[var(--primary-color-3)] p-3",
                          Skeleton { class: "h-5 w-5 flex-none rounded-md" }
                          div { class: "flex min-w-0 flex-1 flex-col gap-2",
                              Skeleton { class: "h-4 w-2/5" }
                              Skeleton { class: "h-3 w-3/5" }
                          }
                          Skeleton { class: "h-7 w-20 flex-none rounded-lg" }
                      }
                  }
              }
          }
      }
  }
}
