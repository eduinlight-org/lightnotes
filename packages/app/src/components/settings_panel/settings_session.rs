use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct SettingsSessionProps {
  pub title: String,
  pub children: Element,
}

#[component]
pub fn SettingsSession(props: SettingsSessionProps) -> Element {
  rsx! {
    div {
        div { class: "mb-2 text-xs font-medium uppercase tracking-wider text-[var(--secondary-color-5)]",
          "{props.title}"
        }
        div { class: "flex flex-col gap-2",
          {props.children}
        }
    }
  }
}
