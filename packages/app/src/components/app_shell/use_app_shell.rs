use crate::state::use_persisted_notes;
use dioxus::prelude::*;

pub fn use_app_shell() {
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
}
