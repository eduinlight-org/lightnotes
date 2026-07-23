use crate::components::EmptyState;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
  rsx! {
      EmptyState {}
  }
}
