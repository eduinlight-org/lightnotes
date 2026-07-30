use dioxus::prelude::*;
use dioxus_icons::lucide::NotebookPen;
use ui::components::button::{Button, ButtonVariant};

mod components;
use components::{Footer, NavBar};

mod theme;
use theme::PRIMARY_BUTTON_CLASS;

mod views;
use views::Home;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

const GITHUB_URL: &str = "https://github.com/eduinlight/lightnotes";

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(LandingNavbar)]
    #[route("/")]
    Home {},
}

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
      document::Link { rel: "icon", href: FAVICON }
      document::Link { rel: "stylesheet", href: ui::THEME_CSS }
      document::Link { rel: "stylesheet", href: MAIN_CSS }
      document::Link { rel: "stylesheet", href: TAILWIND_CSS }
      document::Link { rel: "stylesheet", href: "https://unpkg.com/@phosphor-icons/web@2.1.1/src/regular/style.css" }

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
                      div {
                          class: "flex h-[26px] w-[26px] items-center justify-center rounded-[7px] bg-[#9a8fe0]/15 text-[#9a8fe0]",
                          NotebookPen { size: "16px" }
                      }
                      "LightNotes"
                  }
              },
              actions: rsx! {
                  a {
                      href: GITHUB_URL,
                      class: "no-underline",
                      Button { variant: ButtonVariant::Primary, class: PRIMARY_BUTTON_CLASS, "Get LightNotes" }
                  }
              },
              a { href: "#features", class: "text-white/70 no-underline hover:text-white", "Features" }
              a { href: "#platforms", class: "text-white/70 no-underline hover:text-white", "Platforms" }
              a { href: "#selfhost", class: "text-white/70 no-underline hover:text-white", "Self-hosting" }
              a { href: "#download", class: "text-white/70 no-underline hover:text-white", "Download" }
              a { href: GITHUB_URL, class: "text-white/70 no-underline hover:text-white", "GitHub" }
          }
          main { class: "flex-1", Outlet::<Route> {} }
          Footer {}
      }
  }
}
