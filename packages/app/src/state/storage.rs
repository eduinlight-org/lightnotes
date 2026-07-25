use crate::state::notes::{NotesStore, PersistedState};
use dioxus::prelude::*;

const STORAGE_KEY: &str = "lightnotes:v1";

pub fn use_persisted_notes() -> NotesStore {
  let mut store = use_context_provider(NotesStore::seed);
  let mut loaded = use_signal(|| false);

  use_effect(move || {
    spawn(async move {
      let mut eval = document::eval(&format!(
        "dioxus.send(localStorage.getItem('{STORAGE_KEY}'));"
      ));
      if let Ok(Some(json)) = eval.recv::<Option<String>>().await {
        if let Ok(state) = serde_json::from_str::<PersistedState>(&json) {
          store.restore(state);
        }
      }
      loaded.set(true);
    });
  });

  use_effect(move || {
    let is_loaded = loaded();
    let snapshot = store.snapshot();
    if !is_loaded {
      return;
    }

    spawn(async move {
      let Ok(json) = serde_json::to_string(&snapshot) else {
        return;
      };
      let eval = document::eval(&format!(
        "let json = await dioxus.recv(); localStorage.setItem('{STORAGE_KEY}', json);"
      ));
      let _ = eval.send(json);
    });
  });

  store
}
