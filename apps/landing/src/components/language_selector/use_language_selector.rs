use crate::i18n::{Language, STORAGE_KEY};
use dioxus::prelude::*;

const MOBILE_BREAKPOINT: u32 = 768;

#[derive(Clone, Copy)]
pub struct LanguageSelectorState {
  pub current: Language,
  pub open: Signal<bool>,
  pub is_mobile: Signal<bool>,
}

impl LanguageSelectorState {
  pub fn toggle(&mut self) {
    let open = (self.open)();
    self.open.set(!open);
  }

  pub fn close(&mut self) {
    self.open.set(false);
  }

  pub fn select(&mut self, language: Language) {
    self.open.set(false);

    if language == self.current {
      return;
    }

    let code = language.code();
    spawn(async move {
      let _ = document::eval(&format!(
        "localStorage.setItem('{STORAGE_KEY}', '{code}');
         window.location.assign(window.location.pathname + '?lang={code}' + window.location.hash);"
      ))
      .await;
    });
  }
}

fn use_is_mobile() -> Signal<bool> {
  let mut is_mobile = use_signal(|| false);

  use_effect(move || {
    spawn(async move {
      let mut eval = document::eval(&format!(
        r#"
        const query = window.matchMedia('(max-width: {}px)');
        const send = () => dioxus.send(query.matches);
        query.addEventListener('change', send);
        send();
        "#,
        MOBILE_BREAKPOINT - 1
      ));

      while let Ok(matches) = eval.recv::<bool>().await {
        is_mobile.set(matches);
      }
    });
  });

  is_mobile
}

pub fn use_language_selector() -> LanguageSelectorState {
  LanguageSelectorState { current: use_context::<Language>(), open: use_signal(|| false), is_mobile: use_is_mobile() }
}
