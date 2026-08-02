use dioxus::prelude::*;
use ui::components::skeleton::Skeleton;

#[derive(PartialEq, Clone, Props)]
pub struct NoteListSkeletonProps {
  #[props(default = true)]
  pub with_header: bool,
}

fn row_widths(index: usize) -> (&'static str, &'static str) {
  match index % 3 {
    0 => ("w-3/5", "w-4/5"),
    1 => ("w-2/5", "w-3/5"),
    _ => ("w-1/2", "w-11/12"),
  }
}

#[component]
pub fn NoteListSkeleton(props: NoteListSkeletonProps) -> Element {
  let NoteListSkeletonProps { with_header } = props;

  let wrapper_class = if with_header {
    "flex h-full w-full flex-col border-r border-[var(--primary-color-6)] md:w-80"
  } else {
    "flex h-full w-full flex-col"
  };

  rsx! {
      div { class: wrapper_class, "aria-busy": "true",
          if with_header {
              div { class: "flex flex-none flex-col gap-2 border-b border-[var(--primary-color-6)] p-3",
                  div { class: "flex items-center gap-2",
                      Skeleton { class: "h-7 w-7 flex-none rounded-md" }
                      div { class: "flex min-w-0 flex-1 flex-col gap-1.5",
                          Skeleton { class: "h-4 w-28" }
                          Skeleton { class: "h-3 w-16" }
                      }
                      Skeleton { class: "h-7 w-7 flex-none rounded-md" }
                  }
                  Skeleton { class: "h-9 w-full rounded-lg" }
              }
          }
          div { class: "flex min-h-0 flex-1 flex-col gap-1 overflow-hidden p-2",
              for index in 0..8 {
                  {
                      let (title_width, snippet_width) = row_widths(index);
                      rsx! {
                          div { key: "{index}", class: "flex flex-col gap-2 rounded-lg p-2.5",
                              Skeleton { class: "h-4 {title_width}" }
                              Skeleton { class: "h-3 {snippet_width}" }
                              Skeleton { class: "h-2.5 w-20" }
                          }
                      }
                  }
              }
          }
      }
  }
}
