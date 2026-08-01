use dioxus::prelude::*;
use dioxus_i18n::prelude::{use_init_i18n, I18nConfig};
use unic_langid::{langid, LanguageIdentifier};

const EN_US: &str = include_str!("../i18n/en-US.ftl");
const ES_ES: &str = include_str!("../i18n/es-ES.ftl");

pub const STORAGE_KEY: &str = "lightnotes:landing:lang";

pub const LANGUAGES: [Language; 2] = [Language::English, Language::Spanish];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
  #[default]
  English,
  Spanish,
}

impl Language {
  pub fn code(&self) -> &'static str {
    match self {
      Language::English => "en",
      Language::Spanish => "es",
    }
  }

  pub fn from_code(code: &str) -> Option<Self> {
    match code {
      "en" => Some(Language::English),
      "es" => Some(Language::Spanish),
      _ => None,
    }
  }

  pub fn langid(&self) -> LanguageIdentifier {
    match self {
      Language::English => langid!("en-US"),
      Language::Spanish => langid!("es-ES"),
    }
  }

  pub fn label_key(&self) -> &'static str {
    match self {
      Language::English => "language-en",
      Language::Spanish => "language-es",
    }
  }
}

pub fn use_landing_i18n(lang: &str) -> Language {
  let requested = Language::from_code(lang);
  let language = requested.unwrap_or_default();

  use_init_i18n(move || {
    I18nConfig::new(language.langid())
      .with_locale((langid!("en-US"), EN_US))
      .with_locale((langid!("es-ES"), ES_ES))
      .with_fallback(langid!("en-US"))
  });

  let has_explicit_language = requested.is_some();

  use_effect(move || {
    if has_explicit_language {
      return;
    }

    spawn(async move {
      let mut eval = document::eval(&format!("dioxus.send(localStorage.getItem('{STORAGE_KEY}'));"));
      let Ok(Some(code)) = eval.recv::<Option<String>>().await else {
        return;
      };
      let Some(stored) = Language::from_code(&code) else {
        return;
      };
      if stored == language {
        return;
      }
      let _ = document::eval(&format!(
        "window.location.replace(window.location.pathname + '?lang={}' + window.location.hash);",
        stored.code()
      ))
      .await;
    });
  });

  language
}

#[cfg(test)]
mod tests {
  use super::*;
  use dioxus_i18n::fluent::{FluentBundle, FluentResource};
  use std::collections::BTreeSet;

  fn message_ids(source: &str) -> BTreeSet<String> {
    source
      .lines()
      .filter(|line| !line.starts_with([' ', '#']) && !line.is_empty())
      .filter_map(|line| line.split_once(" ="))
      .map(|(id, _)| id.to_string())
      .collect()
  }

  #[test]
  fn locales_parse() {
    FluentResource::try_new(EN_US.to_string()).expect("en-US should parse");
    FluentResource::try_new(ES_ES.to_string()).expect("es-ES should parse");
  }

  #[test]
  fn locales_expose_the_same_messages() {
    assert_eq!(message_ids(EN_US), message_ids(ES_ES));
  }

  fn assert_every_message_formats(langid: LanguageIdentifier, source: &str) {
    let resource = FluentResource::try_new(source.to_string()).expect("locale should parse");
    let mut bundle = FluentBundle::new(vec![langid]);
    bundle.add_resource(resource).expect("locale should load");

    for id in message_ids(source) {
      let message = bundle.get_message(&id).expect("message should exist");
      let pattern = message.value().expect("message should have a value");
      let mut errors = Vec::new();
      bundle.format_pattern(pattern, None, &mut errors);
      assert!(errors.is_empty(), "{id} failed to format: {errors:?}");
    }
  }

  #[test]
  fn every_message_formats_without_errors() {
    assert_every_message_formats(langid!("en-US"), EN_US);
    assert_every_message_formats(langid!("es-ES"), ES_ES);
  }
}
