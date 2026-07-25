use crate::components::{AppBar, LoginButton};
use crate::state::use_persisted_notes;
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::NotebookText;

#[component]
pub fn AppShell() -> Element {
  let store = use_persisted_notes();

  use_effect(move || {
    let theme_attr = store.theme().as_str();
    spawn(async move {
      let _ = document::eval(&format!(
        "document.documentElement.setAttribute('data-theme', '{theme_attr}');"
      ))
      .await;
    });
  });

  use_effect(move || {
    let accent = store.accent();
    spawn(async move {
      let eval = document::eval(
        "let accent = await dioxus.recv(); document.documentElement.style.setProperty('--accent', accent);",
      );
      let _ = eval.send(accent);
    });
  });

  rsx! {
      div { class: "flex h-dvh flex-col bg-[var(--primary-color-1)]",
          AppBar {
              brand: rsx! {
                  Link {
                      to: Route::Notes {},
                      class: "flex items-center gap-2 whitespace-nowrap text-[var(--secondary-color)] no-underline",
                      NotebookText { size: "18px", stroke: "var(--accent)" }
                      span { "LightNotes" }
                  }
              },
              actions: rsx! { LoginButton {} },
          }
          div { class: "min-h-0 flex-1 overflow-hidden", Outlet::<Route> {} }
      }
  }
}
