use dioxus::prelude::*;
use dioxus_i18n::t;
use ui::components::navbar::Navbar;

#[derive(PartialEq, Clone, Props)]
pub struct AppBarProps {
  pub brand: Element,
  pub children: Element,
  pub actions: Element,
}

#[component]
pub fn AppBar(props: AppBarProps) -> Element {
  let AppBarProps { brand, children, actions } = props;

  rsx! {
      header {
          class: "sticky top-0 z-50 flex items-center gap-2 overflow-x-auto border-b border-[var(--primary-color-6)] bg-[var(--primary-color-1)]/90 px-3 py-3 backdrop-blur sm:gap-6 sm:px-6",
          div { class: "flex items-center gap-2 text-lg font-medium text-[var(--secondary-color)]", {brand} }
          div {
              class: "flex min-w-0 flex-1 justify-center",
              Navbar { aria_label: t!("nav-primary"), {children} }
          }
          div { class: "flex items-center gap-3", {actions} }
      }
  }
}
