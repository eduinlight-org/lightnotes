use dioxus::prelude::*;
use dioxus_icons::lucide::LogIn;
use ui::components::button::{Button, ButtonVariant};

#[component]
pub fn LoginButton(onclick: Option<EventHandler<MouseEvent>>) -> Element {
  rsx! {
      Button { variant: ButtonVariant::Outline, onclick,
          LogIn { size: "16px" }
          span { class: "hidden sm:inline", "Log in" }
      }
  }
}
