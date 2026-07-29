use super::use_date_time_fields::use_date_time_fields;
use crate::state::{date_math, Note};
use dioxus::prelude::*;
use dioxus_icons::lucide::{CalendarDays, Clock};

#[derive(PartialEq, Clone, Props)]
pub struct DateTimeFieldsProps {
  pub note: Note,
}

#[component]
pub fn DateTimeFields(props: DateTimeFieldsProps) -> Element {
  let DateTimeFieldsProps { note } = props;
  let mut fields = use_date_time_fields();
  let date_note_id = note.id.clone();
  let time_note_id = note.id.clone();
  let date_ms = note.date_ms;
  let date_value = date_math::date_ms_to_date_string(date_ms);
  let time_value = date_math::date_ms_to_time_string(date_ms);

  rsx! {
      span { class: "flex h-8 items-center gap-1.5 rounded-md border border-[var(--primary-color-6)] px-2.5",
          CalendarDays { size: "14px", stroke: "var(--accent)" }
          input {
              class: "border-none bg-transparent text-xs text-[var(--secondary-color)] outline-none",
              r#type: "date",
              value: "{date_value}",
              oninput: move |event: FormEvent| fields.set_date(&date_note_id, date_ms, event.value()),
          }
      }
      span { class: "flex h-8 items-center gap-1.5 rounded-md border border-[var(--primary-color-6)] px-2.5",
          Clock { size: "14px", stroke: "var(--accent)" }
          input {
              class: "border-none bg-transparent text-xs text-[var(--secondary-color)] outline-none",
              r#type: "time",
              value: "{time_value}",
              oninput: move |event: FormEvent| fields.set_time(&time_note_id, date_ms, event.value()),
          }
      }
  }
}
