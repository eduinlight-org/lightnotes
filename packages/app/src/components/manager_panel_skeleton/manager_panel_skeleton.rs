use dioxus::prelude::*;
use ui::components::skeleton::Skeleton;

fn chip_width(index: usize) -> &'static str {
  match index % 4 {
    0 => "w-32",
    1 => "w-24",
    2 => "w-40",
    _ => "w-28",
  }
}

#[component]
pub fn ManagerPanelSkeleton() -> Element {
  rsx! {
      div { class: "flex min-h-0 flex-1 flex-col gap-3 overflow-hidden", "aria-busy": "true",
          div { class: "flex flex-none items-center gap-2",
              Skeleton { class: "h-9 flex-1 rounded-lg" }
              Skeleton { class: "h-9 w-9 flex-none rounded-lg" }
          }
          div { class: "flex min-h-0 flex-1 flex-col gap-2 overflow-hidden",
              for index in 0..6 {
                  div { key: "{index}", class: "flex items-center gap-3 rounded-lg p-2.5",
                      Skeleton { class: "h-5 w-5 flex-none rounded-md" }
                      Skeleton { class: "h-4 {chip_width(index)}" }
                      Skeleton { class: "ml-auto h-4 w-8 flex-none" }
                  }
              }
          }
      }
  }
}
