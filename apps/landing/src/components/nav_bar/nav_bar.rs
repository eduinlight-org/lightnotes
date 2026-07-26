use dioxus::prelude::*;
use ui::components::navbar::Navbar;

#[derive(PartialEq, Clone, Props)]
pub struct NavBarProps {
  pub brand: Element,
  pub children: Element,
  pub actions: Element,
}

#[component]
pub fn NavBar(props: NavBarProps) -> Element {
  let NavBarProps { brand, children, actions } = props;

  rsx! {
      header {
          class: "sticky top-0 z-50 flex items-center gap-2 overflow-x-auto border-b border-white/10 bg-[#0f1116]/90 px-3 py-3 backdrop-blur sm:gap-6 sm:px-6",
          div { class: "flex items-center gap-2 text-lg font-bold text-white", {brand} }
          div {
              class: "flex min-w-0 flex-1 justify-center",
              Navbar { aria_label: "Primary", {children} }
          }
          div { class: "flex items-center gap-3", {actions} }
      }
  }
}
