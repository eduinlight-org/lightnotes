use dioxus::prelude::*;
use ui::components::navbar::Navbar;

#[component]
pub fn AppBar(brand: Element, children: Element, actions: Element) -> Element {
  rsx! {
      header {
          class: "sticky top-0 z-50 flex items-center gap-2 overflow-x-auto border-b border-[var(--primary-color-6)] bg-[var(--primary-color-1)]/90 px-3 py-3 backdrop-blur sm:gap-6 sm:px-6",
          div { class: "flex items-center gap-2 text-lg font-medium text-[var(--secondary-color)]", {brand} }
          div {
              class: "flex min-w-0 flex-1 justify-center",
              Navbar { aria_label: "Primary", {children} }
          }
          div { class: "flex items-center gap-3", {actions} }
      }
  }
}
