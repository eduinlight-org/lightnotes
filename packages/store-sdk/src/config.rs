use std::path::PathBuf;

pub struct StoreConfig {
  pub db_path: PathBuf,
  pub api_base_url: String,
}

impl StoreConfig {
  pub fn new(api_base_url: impl Into<String>) -> Self {
    let db_path = dirs::data_dir()
      .unwrap_or_else(std::env::temp_dir)
      .join("lightnotes")
      .join("lightnotes.db");

    Self { db_path, api_base_url: api_base_url.into() }
  }

  pub fn with_db_path(mut self, db_path: PathBuf) -> Self {
    self.db_path = db_path;
    self
  }
}
