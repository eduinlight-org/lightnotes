use dioxus::prelude::*;
use ui::components::skeleton::Skeleton;

#[component]
pub fn DiaryListSkeleton() -> Element {
  rsx! {
      div { class: "flex h-full w-full flex-col overflow-hidden", "aria-busy": "true",
          div { class: "flex flex-none items-center gap-2 p-3",
              Skeleton { class: "h-7 w-7 flex-none rounded-md" }
              Skeleton { class: "h-4 flex-1 max-w-[180px]" }
              Skeleton { class: "h-7 w-7 flex-none rounded-md" }
          }
          div { class: "flex flex-none items-center gap-2 px-3 pb-3",
              Skeleton { class: "h-7 w-20 rounded-lg" }
              Skeleton { class: "h-7 w-20 rounded-lg" }
              Skeleton { class: "h-7 w-20 rounded-lg" }
          }
          div { class: "flex min-h-0 flex-1 flex-col gap-1 overflow-hidden px-3 pb-4",
              for group in 0..3 {
                  div { key: "{group}", class: "flex flex-col gap-1",
                      Skeleton { class: "mt-3 mb-1 h-2.5 w-28" }
                      for row in 0..2 {
                          div { key: "{row}", class: "flex items-start gap-3 rounded-lg p-2.5",
                              Skeleton { class: "h-3 w-10 flex-none" }
                              div { class: "flex min-w-0 flex-1 flex-col gap-2",
                                  Skeleton { class: "h-4 w-1/2" }
                                  Skeleton { class: "h-3 w-4/5" }
                              }
                          }
                      }
                  }
              }
          }
      }
  }
}
