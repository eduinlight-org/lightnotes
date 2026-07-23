use crate::components::{CtaBand, FeatureCards, Hero};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
  rsx! {
      Hero {}
      FeatureCards {}
      CtaBand {}
  }
}
