use crate::components::EmptyState;
use dioxus::prelude::*;

#[component]
pub fn Notes() -> Element {
  rsx! {
      EmptyState {}
  }
}
