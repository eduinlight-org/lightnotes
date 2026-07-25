use dioxus::prelude::*;
use dioxus_icons::lucide::Calendar;

#[component]
pub fn Diary() -> Element {
  rsx! {
      div { class: "flex h-full flex-col items-center justify-center gap-3 px-10 text-center",
          Calendar { size: "50px", stroke: "var(--accent)", class: "opacity-75" }
          h1 { class: "mt-2 text-xl font-medium text-[var(--secondary-color)]", "Diary" }
          p { class: "max-w-sm text-sm leading-relaxed text-[var(--secondary-color-5)]",
              "A day-by-day journal that lives beside your notes — one entry per date, with the same Markdown editor, tags and offline storage."
          }
          div { class: "mt-3 rounded-md border border-[var(--primary-color-6)] px-3 py-1.5 text-[11px] font-medium uppercase tracking-wider text-[var(--secondary-color-6)]",
              "Not built yet"
          }
      }
  }
}
