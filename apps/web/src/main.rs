use app::Route;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!(
  "/assets/main.css",
  AssetOptions::css().with_static_head(true)
);
const TAILWIND_CSS: Asset = asset!(
  "/assets/tailwind.css",
  AssetOptions::css().with_static_head(true)
);

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

      Router::<Route> {}
  }
}
