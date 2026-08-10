use app::Route;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[allow(dead_code)]
fn stylesheets() -> Vec<String> {
  vec![
    ui::THEME_CSS.to_string(),
    MAIN_CSS.to_string(),
    TAILWIND_CSS.to_string(),
  ]
}

const NOTIFY_REMINDER_FLAG: &str = "--notify-reminder";

fn reminder_payload() -> Option<(String, String)> {
  let arguments: Vec<String> = std::env::args().collect();
  let flag = arguments.iter().position(|argument| argument == NOTIFY_REMINDER_FLAG)?;

  Some((
    arguments.get(flag + 1).cloned().unwrap_or_default(),
    arguments.get(flag + 2).cloned().unwrap_or_default(),
  ))
}

fn main() {
  if let Some((title, body)) = reminder_payload() {
    app::deliver_reminder(title, body);
    return;
  }

  dioxus::LaunchBuilder::new()
    .with_cfg(desktop!(dioxus::desktop::Config::new()
      .with_custom_index(app::boot::custom_index(&stylesheets()))
      .with_background_color(app::boot::SPLASH_BACKGROUND_RGBA)))
    .launch(App);
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
