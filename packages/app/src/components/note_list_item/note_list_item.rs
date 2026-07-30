use super::use_note_list_item::use_note_list_item;
use crate::state::{format_relative_time, Note};
use dioxus::prelude::*;
use dioxus_icons::lucide::{BellRing, Pin, Star};

fn snippet(markdown: &str) -> String {
  let plain: String = markdown
    .lines()
    .map(|line| line.trim_start_matches(['#', '-', '*', '>', ' ', '\t']))
    .collect::<Vec<_>>()
    .join(" ");
  plain.chars().take(120).collect()
}

#[derive(PartialEq, Clone, Props)]
pub struct NoteListItemProps {
  pub note: Note,
  pub is_active: bool,
}

#[component]
pub fn NoteListItem(props: NoteListItemProps) -> Element {
  let NoteListItemProps { note, is_active } = props;
  let mut item = use_note_list_item();
  let is_mobile = item.is_mobile;
  let note_id = note.id.clone();
  let star_note_id = note.id.clone();
  let pin_note_id = note.id.clone();
  let snippet_text = snippet(&note.content);
  let tags = item.tags(&note);

  let container_class = if is_mobile() {
    "mb-[9px] flex cursor-pointer flex-col gap-[6px] rounded-[12px] bg-[var(--primary-color-2)] px-[14px] py-[14px] shadow-[0_0_0_1px_var(--primary-color-6)]"
  } else if is_active {
    "mb-1 flex cursor-pointer flex-col gap-[5px] rounded-[9px] border-l-2 border-[var(--accent)] bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] px-[11px] py-[10px]"
  } else {
    "mb-1 flex cursor-pointer flex-col gap-[5px] rounded-[9px] border-l-2 border-transparent px-[11px] py-[10px] hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]"
  };

  let title_class = if is_mobile() {
    "min-w-0 flex-1 truncate text-[16px] font-medium leading-[1.25] text-[var(--secondary-color)]"
  } else {
    "min-w-0 flex-1 truncate text-sm font-medium leading-[1.25] text-[var(--secondary-color)]"
  };

  let icon_size = if is_mobile() { "17px" } else { "14px" };
  let icon_gap_class = if is_mobile() {
    "flex flex-none items-center gap-[10px]"
  } else {
    "flex flex-none items-center gap-[6px]"
  };

  let snippet_class = if is_mobile() {
    "line-clamp-2 text-[13px] leading-[1.45] text-[color-mix(in_srgb,var(--secondary-color)_58%,transparent)]"
  } else {
    "line-clamp-2 text-[12px] leading-[1.45] text-[color-mix(in_srgb,var(--secondary-color)_58%,transparent)]"
  };

  let footer_class = if is_mobile() {
    "flex items-center gap-[8px] text-[11px] text-[color-mix(in_srgb,var(--secondary-color)_45%,transparent)]"
  } else {
    "flex items-center gap-[7px] text-[10.5px] text-[color-mix(in_srgb,var(--secondary-color)_45%,transparent)]"
  };

  rsx! {
      article {
          class: container_class,
          onclick: move |_| item.open(&note_id),
          div { class: "flex items-start gap-2",
              div { class: title_class,
                  if note.title.is_empty() { "Untitled" } else { "{note.title}" }
              }
              div { class: icon_gap_class,
                  button {
                      class: "flex items-center p-0",
                      "aria-label": if note.starred { "Remove from Starred" } else { "Add to Starred" },
                      onclick: move |event: MouseEvent| {
                          event.stop_propagation();
                          item.toggle_star(&star_note_id);
                      },
                      Star {
                          size: icon_size,
                          fill: if note.starred { "#d9b84b" } else { "none" },
                          stroke: if note.starred { "#d9b84b" } else { "color-mix(in srgb,var(--secondary-color) 28%,transparent)" },
                      }
                  }
                  button {
                      class: "flex items-center p-0",
                      "aria-label": if note.pinned { "Unpin from top" } else { "Pin to top of list" },
                      onclick: move |event: MouseEvent| {
                          event.stop_propagation();
                          item.toggle_pin(&pin_note_id);
                      },
                      Pin {
                          size: icon_size,
                          fill: if note.pinned { "var(--accent)" } else { "none" },
                          stroke: if note.pinned { "var(--accent)" } else { "color-mix(in srgb,var(--secondary-color) 28%,transparent)" },
                      }
                  }
              }
          }
          p { class: snippet_class,
              if snippet_text.is_empty() { "No additional text" } else { "{snippet_text}" }
          }
          div { class: footer_class,
              if note.remind_before_hours.is_some() {
                  BellRing { size: "11px", fill: "var(--accent)", stroke: "var(--accent)" }
              }
              span { "{format_relative_time(note.updated_at_ms)}" }
              for tag in tags {
                  span { key: "{tag}", class: "rounded-[5px] border border-[var(--primary-color-6)] px-[6px] py-px", "#{tag}" }
              }
          }
      }
  }
}
