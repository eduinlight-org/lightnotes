use super::use_root_shell::use_root_shell;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn RootShell() -> Element {
  let root = use_root_shell();

  rsx! {
      if !root.gated {
          Outlet::<Route> {}
      }
  }
}
