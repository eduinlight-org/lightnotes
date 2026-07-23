//! This crate contains the shared routes, pages and layout for the workspace.

use dioxus::prelude::*;
use ui::components::navbar::NavbarItem;

mod components;
use components::{AppBar, Footer, LoginButton};

mod views;
pub use views::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppNavbar)]
    #[route("/")]
    Home {},
}

#[component]
fn AppNavbar() -> Element {
  rsx! {
      div {
          class: "flex min-h-screen flex-col bg-[#0f1116]",
          AppBar {
              brand: rsx! {
                  Link {
                      to: Route::Home {},
                      class: "flex items-center gap-2 whitespace-nowrap text-white no-underline",
                      span { "📝" }
                      span { class: "hidden sm:inline", "Notes" }
                  }
              },
              actions: rsx! { LoginButton {} },
              NavbarItem { index: 0usize, value: "home".to_string(), to: Route::Home {}, "Home" }
          }
          main { class: "flex-1", Outlet::<Route> {} }
          Footer {}
      }
  }
}
