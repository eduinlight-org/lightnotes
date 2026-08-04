use dioxus::prelude::*;
use store_sdk::StoreHandle;
use web_time::{SystemTime, UNIX_EPOCH};

use crate::state::auth::{AuthState, PersistedSession};
use crate::state::boot::use_boot;

fn now_ms() -> i64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("system clock before unix epoch")
    .as_millis() as i64
}

pub fn use_persisted_session(mut auth: AuthState) {
  let handle = use_context::<StoreHandle>();
  let mut ready = use_boot().session_ready;

  let restore_handle = handle.clone();
  use_hook(move || {
    spawn(async move {
      if let Some((_, json)) = restore_handle.load_session().await {
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

    let handle = handle.clone();

    spawn(async move {
      let Some((user, tokens)) = user.zip(tokens) else {
        handle.clear_session().await;
        return;
      };

      let user_id = user.id.clone();
      let Ok(json) = serde_json::to_string(&PersistedSession { user, tokens }) else {
        return;
      };

      handle.save_session(&user_id, &json, now_ms()).await;
    });
  });
}
