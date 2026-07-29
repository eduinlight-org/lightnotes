use super::use_reminder_picker::use_reminder_picker;
use crate::components::{ResponsivePopoverContent, ResponsivePopoverRoot, ResponsivePopoverTrigger};
use crate::state::{date_math, Note, REMIND_CHOICES};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Bell, BellRing, ChevronDown};
use ui::components::popover::ContentAlign;

fn remind_label(hours: Option<i64>) -> String {
  match hours {
    None => "Remind me".to_string(),
    Some(0) => "At the time".to_string(),
    Some(h) if h < 24 => format!("{h}h before"),
    Some(h) if h % 168 == 0 => format!("{}w before", h / 168),
    Some(h) => format!("{}d before", h / 24),
  }
}

fn choice_label(hours: Option<i64>) -> &'static str {
  match hours {
    None => "No reminder",
    Some(0) => "At the time",
    Some(1) => "1 hour before",
    Some(2) => "2 hours before",
    Some(3) => "3 hours before",
    Some(6) => "6 hours before",
    Some(12) => "12 hours before",
    Some(24) => "1 day before",
    Some(48) => "2 days before",
    Some(168) => "1 week before",
    _ => "",
  }
}

fn format_absolute(ms: i64) -> String {
  let (_, month, day, hour24, minute) = date_math::date_ms_to_ymdhm(ms);
  let period = if hour24 < 12 { "am" } else { "pm" };
  let hour12 = match hour24 % 12 {
    0 => 12,
    h => h,
  };
  format!("{day} {} \u{b7} {hour12}:{minute:02} {period}", &date_math::month_name(month)[..3])
}

#[derive(PartialEq, Clone, Props)]
pub struct ReminderPickerProps {
  pub note: Note,
}

#[component]
pub fn ReminderPicker(props: ReminderPickerProps) -> Element {
  let ReminderPickerProps { note } = props;
  let mut picker = use_reminder_picker();
  let note_id = note.id.clone();
  let active = note.remind_before_hours.is_some();

  rsx! {
      ResponsivePopoverRoot {
          open: (picker.open)(),
          on_open_change: move |value| picker.open.set(value),
          ResponsivePopoverTrigger {
              class: if active {
                  "flex h-8 items-center gap-1.5 rounded-md border border-[var(--primary-color-6)] px-2.5 text-xs text-[var(--accent)]"
              } else {
                  "flex h-8 items-center gap-1.5 rounded-md border border-[var(--primary-color-6)] px-2.5 text-xs text-[var(--secondary-color)]"
              },
              title: "Reminder",
              if active {
                  BellRing { size: "14px", fill: "var(--accent)", stroke: "var(--accent)" }
              } else {
                  Bell { size: "14px" }
              }
              "{remind_label(note.remind_before_hours)}"
              ChevronDown { size: "11px" }
          }
          ResponsivePopoverContent {
              title: "Remind me",
              align: ContentAlign::Start,
              class: "w-52 items-stretch gap-1 p-1.5 text-left",
              for hours in REMIND_CHOICES {
                  {
                      let note_id = note_id.clone();
                      let is_active = note.remind_before_hours == hours;
                      let class = if is_active {
                          "cursor-pointer rounded-md px-2 py-1.5 text-sm bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] text-[var(--accent)]"
                      } else {
                          "cursor-pointer rounded-md px-2 py-1.5 text-sm text-[var(--secondary-color)] hover:bg-[var(--primary-color-4)]"
                      };
                      rsx! {
                          div {
                              key: "{choice_label(hours)}",
                              class,
                              onclick: move |_| picker.set_remind_before(&note_id, hours),
                              "{choice_label(hours)}"
                          }
                      }
                  }
              }
              if let Some(hours) = note.remind_before_hours {
                  div { class: "mt-1 border-t border-[var(--primary-color-6)] px-2 pt-2 text-[11.5px] text-[color-mix(in_srgb,var(--secondary-color)_55%,transparent)]",
                      "Fires {format_absolute(note.date_ms - hours * date_math::MS_PER_HOUR)}"
                  }
              }
          }
      }
  }
}
