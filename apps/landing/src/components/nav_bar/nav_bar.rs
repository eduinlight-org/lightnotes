use dioxus::prelude::*;

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
          div { class: "flex shrink-0 items-center gap-2 text-lg font-bold text-white", {brand} }
          nav {
              "aria-label": "Primary",
              class: "flex flex-1 shrink-0 items-center justify-center gap-6 whitespace-nowrap text-sm",
              {children}
          }
          div { class: "flex shrink-0 items-center gap-3", {actions} }
      }
  }
}
