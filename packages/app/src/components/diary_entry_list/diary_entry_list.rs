use super::use_diary_entry_list::use_diary_entry_list;
use crate::components::{DiaryCalendar, ResponsivePopoverContent, ResponsivePopoverRoot, ResponsivePopoverTrigger};
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{BellRing, Feather, Funnel, Plus};
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::popover::ContentAlign;

const TRIGGER_BUTTON_CLASS: &str = "flex h-8 w-8 flex-none items-center justify-center rounded-md border border-[var(--primary-color-6)] p-0 text-[var(--secondary-color)] hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]";
const TRIGGER_BUTTON_ACTIVE_CLASS: &str = "flex h-8 w-8 flex-none items-center justify-center rounded-md border border-[var(--primary-color-6)] p-0 text-[var(--accent)] hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]";

fn article_class(is_mobile: bool, is_active: bool) -> &'static str {
  if is_mobile {
    "mb-2 flex cursor-pointer flex-col gap-1.5 rounded-[12px] bg-[var(--primary-color-2)] px-[14px] py-[14px] shadow-[0_0_0_1px_var(--primary-color-6)]"
  } else if is_active {
    "mb-0.5 flex cursor-pointer flex-col gap-1 rounded-md border-l-2 border-[var(--accent)] bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] px-2.5 py-2"
  } else {
    "mb-0.5 flex cursor-pointer flex-col gap-1 rounded-md border-l-2 border-transparent px-2.5 py-2 hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]"
  }
}

fn filter_row_class(active: bool) -> &'static str {
  if active {
    "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1.5 text-sm bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] text-[var(--accent)]"
  } else {
    "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1.5 text-sm text-[var(--secondary-color)] hover:bg-[var(--primary-color-4)]"
  }
}

#[component]
pub fn DiaryEntryList() -> Element {
  let mut list = use_diary_entry_list();
  let active_note_id = match use_route::<Route>() {
    Route::DiaryEntry { note_id } => Some(note_id),
    _ => None,
  };
  let entries = list.entries(active_note_id.as_deref());
  let is_mobile = list.is_mobile;
  let has_filter = list.filter_folder().is_some() || list.filter_tag().is_some();
  let count_label = if entries.len() == 1 { "1 note".to_string() } else { format!("{} notes", entries.len()) };
  let sub_label = format!("{count_label} \u{b7} {}", list.filter_summary());

  rsx! {
      div { class: "flex flex-none items-center gap-1.5 px-3 py-2.5",
          div { class: "min-w-0 flex-1",
              div { class: "truncate text-sm font-medium text-[var(--secondary-color)]", "{list.period_label()}" }
              div { class: "truncate text-xs text-[color-mix(in_srgb,var(--secondary-color)_55%,transparent)]", "{sub_label}" }
          }
          DiaryCalendar {}
          ResponsivePopoverRoot {
              open: list.filter_open(),
              on_open_change: move |value| list.set_filter_open(value),
              ResponsivePopoverTrigger {
                  class: if has_filter { TRIGGER_BUTTON_ACTIVE_CLASS } else { TRIGGER_BUTTON_CLASS },
                  title: "Filter by folder or tag",
                  Funnel { size: "15px" }
              }
              ResponsivePopoverContent {
                  title: "Filter",
                  align: ContentAlign::End,
                  class: "w-56 items-stretch gap-1 p-1.5 text-left",
                  if has_filter {
                      div {
                          class: "flex cursor-pointer items-center justify-end px-2 py-1 text-xs text-[var(--accent)]",
                          onclick: move |_| list.clear_filters(),
                          "Clear"
                      }
                  }
                  div {
                      class: filter_row_class(list.filter_folder().is_none()),
                      onclick: move |_| list.set_filter_folder(None),
                      "All folders"
                  }
                  for folder in list.folders() {
                      {
                          let folder_id = folder.id.clone();
                          let active = list.filter_folder().as_deref() == Some(folder.id.as_str());
                          rsx! {
                              div {
                                  key: "{folder.id}",
                                  class: filter_row_class(active),
                                  onclick: move |_| list.set_filter_folder(Some(folder_id.clone())),
                                  "{folder.name}"
                              }
                          }
                      }
                  }
                  div { class: "my-1 h-px bg-[var(--primary-color-6)]" }
                  div {
                      class: filter_row_class(list.filter_tag().is_none()),
                      onclick: move |_| list.set_filter_tag(None),
                      "All tags"
                  }
                  for tag in list.tags() {
                      {
                          let tag_id = tag.id.clone();
                          let active = list.filter_tag().as_deref() == Some(tag.id.as_str());
                          rsx! {
                              div {
                                  key: "{tag.id}",
                                  class: filter_row_class(active),
                                  onclick: move |_| list.set_filter_tag(Some(tag_id.clone())),
                                  "#{tag.name}"
                              }
                          }
                      }
                  }
              }
          }
          Button {
              variant: ButtonVariant::Primary,
              size: ButtonSize::IconSm,
              class: "border border-[var(--accent)] bg-transparent text-[var(--accent)] hover:bg-[color-mix(in_srgb,var(--accent)_12%,transparent)]",
              "aria-label": "New note",
              onclick: move |_| list.create_entry(),
              Plus { size: "16px" }
          }
      }
      div { class: "min-h-0 flex-1 overflow-y-auto px-2 pb-3",
          if entries.is_empty() {
              div { class: "flex flex-col items-center gap-2 py-10 text-center text-[color-mix(in_srgb,var(--secondary-color)_55%,transparent)]",
                  Feather { size: "30px", class: "opacity-50" }
                  div { class: "text-sm", "Nothing written here yet." }
              }
          }
          for entry in entries {
              {
                  let note_id = entry.id.clone();
                  let mobile = is_mobile();
                  let title_class = if mobile {
                    "min-w-0 flex-1 truncate text-[15px] font-medium text-[var(--secondary-color)]"
                  } else {
                    "min-w-0 flex-1 truncate text-[13.5px] font-medium text-[var(--secondary-color)]"
                  };
                  let snippet_class = if mobile {
                    "line-clamp-2 text-[13px] leading-[1.45] text-[color-mix(in_srgb,var(--secondary-color)_56%,transparent)]"
                  } else {
                    "line-clamp-2 text-xs leading-[1.45] text-[color-mix(in_srgb,var(--secondary-color)_56%,transparent)]"
                  };
                  rsx! {
                      div { key: "{entry.id}",
                          if entry.show_day_header {
                              div { class: "px-2 pb-1 pt-2.5 text-[10px] uppercase tracking-wider text-[color-mix(in_srgb,var(--secondary-color)_42%,transparent)]",
                                  "{entry.day_header_label}"
                              }
                          }
                          article {
                              class: article_class(mobile, entry.is_active),
                              onclick: move |_| list.open(&note_id),
                              div { class: "flex items-baseline gap-2",
                                  span { class: "flex-none text-[11px] font-medium tabular-nums text-[var(--accent)]", "{entry.time_label}" }
                                  span { class: title_class, "{entry.title}" }
                                  if entry.has_reminder {
                                      BellRing { size: "11px", fill: "var(--accent)", stroke: "var(--accent)" }
                                  }
                                  if !mobile {
                                      span { class: "flex-none text-[10px] text-[color-mix(in_srgb,var(--secondary-color)_38%,transparent)]", "{entry.folder_name}" }
                                  }
                              }
                              p { class: snippet_class,
                                  if entry.snippet.is_empty() { "Empty note" } else { "{entry.snippet}" }
                              }
                          }
                      }
                  }
              }
          }
      }
  }
}
