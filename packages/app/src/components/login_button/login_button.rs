use dioxus::prelude::*;
use dioxus_icons::lucide::LogIn;
use ui::components::button::{Button, ButtonVariant};

#[derive(PartialEq, Clone, Props)]
pub struct LoginButtonProps {
  pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn LoginButton(props: LoginButtonProps) -> Element {
  let LoginButtonProps { onclick } = props;

  rsx! {
      Button { variant: ButtonVariant::Outline, onclick,
          LogIn { size: "16px" }
          span { class: "hidden sm:inline", "Log in" }
      }
  }
}
