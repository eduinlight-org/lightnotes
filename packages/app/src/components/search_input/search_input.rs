use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_icons::lucide::{Search, X};
use ui::components::input::Input;

#[derive(Copy, Clone, PartialEq, Default)]
pub enum SearchInputSize {
  #[default]
  Default,
  Large,
}

#[derive(PartialEq, Clone, Props)]
pub struct SearchInputProps {
  pub value: String,
  pub on_search: EventHandler<String>,
  #[props(default)]
  pub size: SearchInputSize,
}

#[component]
pub fn SearchInput(props: SearchInputProps) -> Element {
  let SearchInputProps { value, on_search, size } = props;
  let has_value = !value.trim().is_empty();
  let (wrapper_class, icon_size, text_size, clear_size) = match size {
    SearchInputSize::Default => (
      "flex h-[34px] items-center gap-2 rounded-lg border border-[var(--primary-color-6)] bg-[var(--primary-color-2)] px-[11px] focus-within:border-[var(--accent)]",
      "15px",
      "13px",
      "14px",
    ),
    SearchInputSize::Large => (
      "flex h-10 items-center gap-2 rounded-xl border border-[var(--primary-color-6)] bg-[var(--primary-color-2)] px-3 focus-within:border-[var(--accent)]",
      "17px",
      "14.5px",
      "16px",
    ),
  };

  rsx! {
      div { class: wrapper_class,
          Search { class: "flex-none text-[var(--secondary-color-5)]", size: icon_size }
          Input {
              class: format!(
                  "h-full flex-1 border-none bg-transparent p-0 text-[{text_size}] shadow-none [outline:none] hover:bg-transparent focus:bg-transparent focus:shadow-none",
              ),
              placeholder: t!("notes-search-placeholder"),
              value,
              oninput: move |event: FormEvent| on_search.call(event.value()),
          }
          if has_value {
              button {
                  class: "flex flex-none items-center text-[var(--secondary-color-5)]",
                  "aria-label": t!("notes-clear-search"),
                  onclick: move |_| on_search.call(String::new()),
                  X { size: clear_size }
              }
          }
      }
  }
}
