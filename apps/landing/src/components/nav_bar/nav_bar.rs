use dioxus::prelude::*;
use dioxus_i18n::t;

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
          class: "nav",
          div { class: "nav-brand flex shrink-0 items-center", {brand} }
          nav {
              "aria-label": t!("nav-primary"),
              class: "flex items-center gap-7 max-md:hidden",
              {children}
          }
          div { class: "flex shrink-0 items-center gap-3", {actions} }
      }
  }
}
