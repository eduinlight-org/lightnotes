use serde::{Deserialize, Serialize};
use unic_langid::{langid, LanguageIdentifier};

pub const LANGUAGES: [Language; 2] = [Language::English, Language::Spanish];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
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
