use dioxus::prelude::*;
use dioxus_i18n::t;

mod components;
use components::{Footer, LanguageSelector, NavBar};

mod config;

mod i18n;
use i18n::use_landing_i18n;

mod views;
use views::Home;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const BRAND_ICON: Asset = asset!("/assets/notes-icon.png");

const INTER_CSS: &str = "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap";
const PHOSPHOR_CSS: &str = "https://unpkg.com/@phosphor-icons/web@2.1.1/src/regular/style.css";

const NAV_LINKS: [(&str, &str); 5] = [
  ("#features", "nav-features"),
  ("#platforms", "nav-platforms"),
  ("#selfhost", "nav-self-hosting"),
  ("#download", "nav-download"),
  (config::GITHUB_URL, "nav-github"),
];

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(LandingNavbar)]
    #[route("/?:lang")]
    Home { lang: String },
}

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
      document::Link { rel: "icon", href: FAVICON }
      document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous" }
      document::Link { rel: "stylesheet", href: INTER_CSS }
      document::Link { rel: "stylesheet", href: PHOSPHOR_CSS }
      document::Link { rel: "stylesheet", href: TAILWIND_CSS }

      Router::<Route> {}
  }
}

#[component]
fn LandingNavbar() -> Element {
  let Route::Home { lang } = use_route::<Route>();
  let language = use_landing_i18n(&lang);
  use_context_provider(|| language);

  rsx! {
      div {
          class: "flex min-h-screen flex-col",
          NavBar {
              brand: rsx! {
                  Link {
                      to: Route::Home { lang: language.code().to_string() },
                      class: "flex items-center gap-2.5",
                      img { src: BRAND_ICON, alt: "", class: "h-[26px] w-[26px] rounded-[7px]" }
                      {t!("brand-name")}
                  }
              },
              actions: rsx! {
                  LanguageSelector {}
                  a { class: "btn btn-primary", href: config::APP_URL, {t!("nav-cta")} }
              },
              for (href , label_key) in NAV_LINKS {
                  a { href, {t!(label_key)} }
              }
          }
          main { class: "flex-1", Outlet::<Route> {} }
          Footer {}
      }
  }
}
