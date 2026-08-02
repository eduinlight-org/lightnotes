use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct BootState {
  pub prefs_ready: Signal<bool>,
  pub store_ready: Signal<bool>,
  pub viewport_ready: Signal<bool>,
}

impl BootState {
  pub fn seed() -> Self {
    Self {
      prefs_ready: Signal::new(false),
      store_ready: Signal::new(false),
      viewport_ready: Signal::new(false),
    }
  }

  pub fn ready(&self) -> bool {
    (self.prefs_ready)() && (self.store_ready)() && (self.viewport_ready)()
  }
}

pub fn use_boot() -> BootState {
  use_context()
}
