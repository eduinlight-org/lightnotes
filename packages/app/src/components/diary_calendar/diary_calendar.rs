use super::use_diary_calendar::{use_diary_calendar, DayCell};
use crate::components::{ResponsivePopoverContent, ResponsivePopoverRoot, ResponsivePopoverTrigger};
use crate::state::{i18n, CalendarViewMode};
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_icons::lucide::{BellRing, CalendarDays, ChevronLeft, ChevronRight};
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::popover::ContentAlign;

const TRIGGER_BUTTON_CLASS: &str = "flex h-8 w-8 flex-none items-center justify-center rounded-md border border-[var(--primary-color-6)] p-0 text-[var(--secondary-color)] hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]";

fn segmented_class(active: bool, mobile: bool) -> String {
  let padding = if mobile { "py-2" } else { "py-1" };
  if active {
    format!("flex-1 rounded-md {padding} bg-[color-mix(in_srgb,var(--accent)_18%,transparent)] px-0 text-xs font-medium text-[var(--accent)]")
  } else {
    format!("flex-1 rounded-md {padding} px-0 text-xs font-medium text-[color-mix(in_srgb,var(--secondary-color)_62%,transparent)]")
  }
}

fn cell_class(cell: &DayCell, height_class: &str) -> String {
  let base = format!("flex {height_class} flex-col items-center justify-center gap-0.5 rounded-md cursor-pointer");
  let ring = if cell.is_today { " ring-1 ring-[var(--accent)]" } else { "" };
  if cell.is_selected {
    format!("{base}{ring} bg-[color-mix(in_srgb,var(--accent)_22%,transparent)] text-[var(--accent)]")
  } else if !cell.in_current_month {
    format!("{base}{ring} text-[color-mix(in_srgb,var(--secondary-color)_28%,transparent)]")
  } else {
    format!("{base}{ring} text-[var(--secondary-color)] hover:bg-[color-mix(in_srgb,var(--secondary-color)_8%,transparent)]")
  }
}

#[component]
pub fn DiaryCalendar() -> Element {
  let mut calendar = use_diary_calendar();
  let view_mode = calendar.view_mode();
  let cells = if view_mode == CalendarViewMode::Month { calendar.month_cells() } else { calendar.week_cells() };
  let is_mobile = calendar.is_mobile;
  let mobile = is_mobile();

  rsx! {
      ResponsivePopoverRoot {
          open: calendar.calendar_open(),
          on_open_change: move |value| calendar.set_calendar_open(value),
          ResponsivePopoverTrigger {
              class: TRIGGER_BUTTON_CLASS,
              title: t!("calendar-title"),
              CalendarDays { size: "15px" }
          }
          ResponsivePopoverContent {
              title: t!("calendar-title"),
              align: ContentAlign::End,
              class: "w-44 items-stretch gap-0 p-2 text-left",
              div { class: "flex items-center gap-1",
                  Button {
                      variant: ButtonVariant::Ghost,
                      size: ButtonSize::IconSm,
                      "aria-label": t!("action-previous"),
                      onclick: move |_| calendar.step(-1),
                      ChevronLeft { size: "15px" }
                  }
                  div { class: "flex-1 truncate text-center text-sm font-medium text-[var(--secondary-color)]", "{calendar.header_label()}" }
                  Button {
                      variant: ButtonVariant::Ghost,
                      size: ButtonSize::IconSm,
                      "aria-label": t!("action-next"),
                      onclick: move |_| calendar.step(1),
                      ChevronRight { size: "15px" }
                  }
              }
              if mobile {
                  div { class: "mt-2.5 grid grid-cols-7 gap-0.5",
                      for cell in cells {
                          {
                              let day_key = cell.day_key;
                              let class = cell_class(&cell, "h-11");
                              rsx! {
                                  div { key: "{day_key}", class, onclick: move |_| calendar.select_day(day_key),
                                      span { class: "text-[9px] uppercase tracking-wider opacity-55", "{cell.dow_label}" }
                                      span { class: "text-[13px]", "{cell.label}" }
                                      span { class: "flex h-2 items-center justify-center gap-0.5",
                                          span { class: if cell.has_notes { "h-1 w-1 rounded-full bg-[var(--accent)]" } else { "h-1 w-1 rounded-full bg-transparent" } }
                                          if cell.has_reminder {
                                              BellRing { size: "8px", fill: "var(--accent)", stroke: "var(--accent)" }
                                          }
                                      }
                                  }
                              }
                          }
                      }
                  }
              } else {
                  div { class: "mt-2 grid grid-cols-7 gap-0.5",
                      for i in 0..7u32 {
                          span { key: "{i}", class: "text-center text-[8px] uppercase tracking-wider opacity-55", {i18n::weekday_narrow_name(i)} }
                      }
                  }
                  div { class: "mt-0.5 grid grid-cols-7 gap-0.5",
                      for cell in cells {
                          {
                              let day_key = cell.day_key;
                              let class = cell_class(&cell, "h-7");
                              rsx! {
                                  div { key: "{day_key}", class, onclick: move |_| calendar.select_day(day_key),
                                      span { class: "text-[11px]", "{cell.label}" }
                                      span { class: "flex h-1.5 items-center justify-center gap-0.5",
                                          span { class: if cell.has_notes { "h-1 w-1 rounded-full bg-[var(--accent)]" } else { "h-1 w-1 rounded-full bg-transparent" } }
                                          if cell.has_reminder {
                                              BellRing { size: "7px", fill: "var(--accent)", stroke: "var(--accent)" }
                                          }
                                      }
                                  }
                              }
                          }
                      }
                  }
              }
              div { class: "mt-2 flex gap-0.5 rounded-lg bg-[color-mix(in_srgb,var(--secondary-color)_6%,transparent)] p-0.5",
                  button { class: segmented_class(view_mode == CalendarViewMode::Day, mobile), onclick: move |_| calendar.set_view(CalendarViewMode::Day), {t!("calendar-day")} }
                  button { class: segmented_class(view_mode == CalendarViewMode::Week, mobile), onclick: move |_| calendar.set_view(CalendarViewMode::Week), {t!("calendar-week")} }
                  button { class: segmented_class(view_mode == CalendarViewMode::Month, mobile), onclick: move |_| calendar.set_view(CalendarViewMode::Month), {t!("calendar-month")} }
              }
          }
      }
  }
}
