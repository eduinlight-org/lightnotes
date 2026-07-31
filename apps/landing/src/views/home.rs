use crate::components::{CtaBand, DownloadSection, FeatureList, Hero, PlatformsSection, SelfHostSection, StatsBand};
use dioxus::prelude::*;

#[component]
pub fn Home(lang: String) -> Element {
  let _ = lang;

  rsx! {
      Hero {}
      StatsBand {}
      FeatureList {}
      PlatformsSection {}
      SelfHostSection {}
      DownloadSection {}
      CtaBand {}
  }
}
