use std::sync::Arc;
use std::time::Duration;

use api_sdk::ApiClient;
use dioxus::prelude::*;

use super::use_login_button::LoginButtonState;
use crate::state::{api_base_url, use_auth};

const BASE_POLL_INTERVAL_MS: u64 = 2000;
const MAX_POLL_INTERVAL_MS: u64 = 10_000;
const POLL_TIMEOUT_MS: u64 = 3 * 60 * 1000;

fn open_in_system_browser(url: &str) -> bool {
  eprintln!("LIGHTNOTES_AUTH: opening browser for {url}");

  match webbrowser::open(url) {
    Ok(()) => {
      eprintln!("LIGHTNOTES_AUTH: browser open reported success");
      true
    }
    Err(err) => {
      eprintln!("LIGHTNOTES_AUTH: browser open failed: kind={:?} err={err}", err.kind());
      dioxus::logger::tracing::error!("could not open the system browser: {err}");
      false
    }
  }
}

pub fn use_login_button() -> LoginButtonState {
  let mut auth = use_auth();
  let ready = use_signal(|| true);
  let mut failed = use_signal(|| false);
  let mut pending = use_signal(|| false);

  let start = use_callback(move |_| {
    if *pending.peek() {
      return;
    }

    pending.set(true);
    failed.set(false);

    spawn(async move {
      let api = Arc::new(ApiClient::new(api_base_url()));

      let started = match api.native_auth_start().await {
        Ok(started) => started,
        Err(err) => {
          eprintln!("LIGHTNOTES_AUTH: native_auth_start failed: {err}");
          failed.set(true);
          pending.set(false);
          return;
        }
      };

      if !open_in_system_browser(&started.authorize_url) {
        failed.set(true);
        pending.set(false);
        return;
      }

      let mut waited_ms = 0u64;
      let mut interval_ms = BASE_POLL_INTERVAL_MS;

      loop {
        tokio::time::sleep(Duration::from_millis(interval_ms)).await;
        waited_ms += interval_ms;
        interval_ms = (interval_ms * 3 / 2).min(MAX_POLL_INTERVAL_MS);

        match api.native_auth_poll(&started.ticket).await {
          Ok(Some((user, tokens))) => {
            auth.sign_in(user, tokens);
            pending.set(false);
            return;
          }
          Ok(None) => {}
          Err(_) => {
            failed.set(true);
            pending.set(false);
            return;
          }
        }

        if waited_ms >= POLL_TIMEOUT_MS {
          failed.set(true);
          pending.set(false);
          return;
        }
      }
    });
  });

  LoginButtonState {
    ready,
    failed,
    pending,
    start: start.into(),
  }
}
