//! This crate contains the shared routes, pages and layout for the workspace.

use dioxus::prelude::*;
use ui::Navbar;

mod views;
pub use views::{Blog, Home};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const NAV_LINK_CLASS: &str = "text-white no-underline mr-5 transition-colors duration-200 hover:cursor-pointer hover:text-[#91a4d2]";

#[component]
fn AppNavbar() -> Element {
  rsx! {
      Navbar {
          Link {
              to: Route::Home {},
              class: NAV_LINK_CLASS,
              "Home"
          }
          Link {
              to: Route::Blog { id: 1 },
              class: NAV_LINK_CLASS,
              "Blog"
          }
      }

      Outlet::<Route> {}
  }
}
