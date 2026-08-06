use std::sync::Arc;

use api_sdk::ApiClient;
use dioxus::prelude::*;

use super::use_login_button::LoginButtonState;
use crate::state::{api_base_url, use_auth};

const READY_MARKER: &str = "__ready__";

const BRIDGE_JS: &str = r#"
const clientId = await dioxus.recv();

if (!(window.google && window.google.accounts && window.google.accounts.id)) {
  await new Promise((resolve, reject) => {
    const src = 'https://accounts.google.com/gsi/client';
    const existing = document.querySelector(`script[src="${src}"]`);
    if (existing) {
      existing.addEventListener('load', resolve);
      existing.addEventListener('error', reject);
      return;
    }
    const script = document.createElement('script');
    script.src = src;
    script.async = true;
    script.defer = true;
    script.onload = resolve;
    script.onerror = reject;
    document.head.appendChild(script);
  });
}

let target = null;
for (let attempt = 0; attempt < 50 && !target; attempt += 1) {
  target = document.getElementById('google-signin-button');
  if (!target) {
    await new Promise((resolve) => setTimeout(resolve, 50));
  }
}

if (target) {
  window.google.accounts.id.initialize({
    client_id: clientId,
    callback: (response) => dioxus.send(response.credential),
  });
  window.google.accounts.id.renderButton(target, {
    theme: 'outline',
    size: 'large',
    shape: 'pill',
    text: 'signin_with',
    width: 280,
  });
  dioxus.send('__ready__');
}
"#;

pub fn use_login_button() -> LoginButtonState {
  let mut auth = use_auth();
  let mut ready = use_signal(|| false);
  let mut failed = use_signal(|| false);
  let pending = use_signal(|| false);

  use_hook(move || {
    spawn(async move {
      let api = Arc::new(ApiClient::new(api_base_url()));

      let Ok(config) = api.auth_config().await else {
        failed.set(true);
        return;
      };

      let mut eval = document::eval(BRIDGE_JS);

      if eval.send(config.google_client_id).is_err() {
        failed.set(true);
        return;
      }

      while let Ok(message) = eval.recv::<String>().await {
        if message == READY_MARKER {
          ready.set(true);
          continue;
        }

        let device_id = uuid::Uuid::new_v4().to_string();

        match api.google_sign_in(message, device_id).await {
          Ok((user, tokens)) => {
            failed.set(false);
            auth.sign_in(user, tokens);
          }
          Err(_) => failed.set(true),
        }
      }
    });
  });

  LoginButtonState {
    ready,
    failed,
    pending,
    start: Callback::new(|_| {}),
  }
}
