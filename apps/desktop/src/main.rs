use app::Route;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

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
