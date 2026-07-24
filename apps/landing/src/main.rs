use dioxus::prelude::*;
use dioxus_icons::lucide::NotebookPen;
use ui::components::button::{Button, ButtonVariant};
use ui::components::navbar::NavbarItem;

mod components;
use components::{Footer, NavBar};

mod views;
use views::{Features, Home};

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(LandingNavbar)]
    #[route("/")]
    Home {},
    #[route("/features")]
    Features {},
}

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
      document::Link { rel: "stylesheet", href: ui::THEME_CSS }
      document::Link { rel: "stylesheet", href: MAIN_CSS }
      document::Link { rel: "stylesheet", href: TAILWIND_CSS }

      Router::<Route> {}
  }
}

#[component]
fn LandingNavbar() -> Element {
  rsx! {
      div {
          class: "flex min-h-screen flex-col bg-[#0f1116]",
          NavBar {
              brand: rsx! {
                  Link {
                      to: Route::Home {},
                      class: "flex items-center gap-2 whitespace-nowrap text-white no-underline",
                      NotebookPen { size: "18px" }
                      span { class: "hidden sm:inline", "Notes" }
                  }
              },
              actions: rsx! {
                  Link {
                      to: Route::Home {},
                      class: "no-underline",
                      Button { variant: ButtonVariant::Primary, "Open app" }
                  }
              },
              NavbarItem { index: 0usize, value: "home".to_string(), to: Route::Home {}, "Home" }
              NavbarItem { index: 1usize, value: "features".to_string(), to: Route::Features {}, "Features" }
          }
          main { class: "flex-1", Outlet::<Route> {} }
          Footer {}
      }
  }
}
