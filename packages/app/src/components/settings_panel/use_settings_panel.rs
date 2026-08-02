use crate::state::{use_boot, use_notes, NotesStore, Theme};

#[derive(Clone, Copy)]
pub struct SettingsPanelState {
  pub store: NotesStore,
  pub ready: bool,
}

impl SettingsPanelState {
  pub fn set_theme(&mut self, theme: Theme) {
    self.store.set_theme(theme);
  }

  pub fn set_accent(&mut self, accent: String) {
    self.store.set_accent(accent);
  }

  pub fn toggle_sync(&mut self) {
    self.store.toggle_sync();
  }
}

pub fn use_settings_panel() -> SettingsPanelState {
  SettingsPanelState {
    store: use_notes(),
    ready: use_boot().ready(),
  }
}
