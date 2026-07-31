use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::language::Language;
use super::notes::{NotesStore, Theme};

const PREFS_KEY: &str = "lightnotes:prefs:v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Prefs {
  theme: Theme,
  accent: String,
  #[serde(default)]
  language: Language,
}

pub fn use_persisted_preferences(mut store: NotesStore) {
  let mut loaded = use_signal(|| false);

  use_effect(move || {
    spawn(async move {
      let mut eval = document::eval(&format!("dioxus.send(localStorage.getItem('{PREFS_KEY}'));"));
      if let Ok(Some(json)) = eval.recv::<Option<String>>().await {
        if let Ok(prefs) = serde_json::from_str::<Prefs>(&json) {
          store.set_theme(prefs.theme);
          store.set_accent(prefs.accent);
          store.set_language(prefs.language);
        }
      }
      loaded.set(true);
    });
  });

  use_effect(move || {
    let is_loaded = loaded();
    let prefs = Prefs { theme: store.theme(), accent: store.accent(), language: store.language() };

    if !is_loaded {
      return;
    }

    spawn(async move {
      let Ok(json) = serde_json::to_string(&prefs) else {
        return;
      };
      let eval = document::eval(&format!(
        "let json = await dioxus.recv(); localStorage.setItem('{PREFS_KEY}', json);"
      ));
      let _ = eval.send(json);
    });
  });
}
