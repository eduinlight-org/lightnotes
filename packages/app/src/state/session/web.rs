use dioxus::prelude::*;

use crate::state::auth::{AuthState, PersistedSession};
use crate::state::boot::use_boot;

const SESSION_KEY: &str = "lightnotes:session:v1";

pub fn use_persisted_session(mut auth: AuthState) {
  let mut ready = use_boot().session_ready;

  use_effect(move || {
    spawn(async move {
      let mut eval = document::eval(&format!("dioxus.send(localStorage.getItem('{SESSION_KEY}'));"));

      if let Ok(Some(json)) = eval.recv::<Option<String>>().await {
        if let Ok(session) = serde_json::from_str::<PersistedSession>(&json) {
          auth.restore(session);
        }
      }

      auth.settle_restore();
      ready.set(true);
    });
  });

  use_effect(move || {
    let is_ready = ready();
    let user = auth.user();
    let tokens = auth.tokens();

    if !is_ready {
      return;
    }

    spawn(async move {
      let Some((user, tokens)) = user.zip(tokens) else {
        let _ = document::eval(&format!("localStorage.removeItem('{SESSION_KEY}');")).await;
        return;
      };

      let Ok(json) = serde_json::to_string(&PersistedSession { user, tokens }) else {
        return;
      };

      let eval = document::eval(&format!(
        "let json = await dioxus.recv(); localStorage.setItem('{SESSION_KEY}', json);"
      ));
      let _ = eval.send(json);
    });
  });
}
