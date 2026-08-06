use crate::components::LoginScreen;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
  rsx! {
      LoginScreen {}
  }
}
