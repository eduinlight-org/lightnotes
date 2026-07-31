use super::use_settings_panel::use_settings_panel;
use super::SettingsSession;
use crate::components::LanguagePicker;
use crate::state::{SyncStatus, Theme, ACCENT_SWATCHES};
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_icons::lucide::{Check, CloudCheck, CloudOff, HardDrive, Moon, Notebook, Sun};
use ui::components::button::{Button, ButtonSize, ButtonVariant};

fn theme_card_class(active: bool) -> &'static str {
  if active {
    "flex flex-1 flex-col items-center gap-2 rounded-lg border border-[var(--accent)] bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] p-4 text-[var(--accent)]"
  } else {
    "flex flex-1 flex-col items-center gap-2 rounded-lg border border-[var(--primary-color-6)] bg-transparent p-4 text-[var(--secondary-color)]"
  }
}

#[component]
pub fn SettingsPanel() -> Element {
  let mut settings = use_settings_panel();
  let store = settings.store;
  let theme = store.theme();
  let accent = store.accent();
  let sync = store.sync();
  let note_count = store.note_count();

  let sync_icon_color = "var(--secondary-color-5)";
  let sync_label = match sync {
    SyncStatus::Synced => t!("sync-saved"),
    SyncStatus::Offline => t!("sync-offline"),
  };

  rsx! {
      div { class: "flex flex-col gap-5",
          SettingsSession { title: t!("settings-appearance"),
              div { class: "flex gap-2",
                  button {
                      class: theme_card_class(theme == Theme::Dark),
                      onclick: move |_| settings.set_theme(Theme::Dark),
                      Moon { size: "20px" }
                      span { class: "text-sm font-medium", {t!("settings-theme-dark")} }
                  }
                  button {
                      class: theme_card_class(theme == Theme::Light),
                      onclick: move |_| settings.set_theme(Theme::Light),
                      Sun { size: "20px" }
                      span { class: "text-sm font-medium", {t!("settings-theme-light")} }
                  }
              }
              div { class: "flex items-center gap-4 rounded-lg bg-[var(--primary-color-3)] p-3",
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", {t!("settings-accent")} }
                      div { class: "text-xs text-[var(--secondary-color-5)]", {t!("settings-accent-description")} }
                  }
                  div { class: "flex gap-2.5",
                      for hex in ACCENT_SWATCHES {
                          {
                              let is_active = accent.eq_ignore_ascii_case(hex);
                              let style = format!(
                                  "width:24px;height:24px;border-radius:9999px;cursor:pointer;background:{hex};box-shadow:{}",
                                  if is_active { format!("0 0 0 2px var(--primary-color-3), 0 0 0 4px {hex}") } else { "none".to_string() },
                              );
                              rsx! {
                                  span {
                                      key: "{hex}",
                                      style,
                                      onclick: move |_| settings.set_accent(hex.to_string()),
                                  }
                              }
                          }
                      }
                  }
              }
          }
          SettingsSession { title: t!("settings-language"),
              div { class: "flex items-center gap-4 rounded-lg bg-[var(--primary-color-3)] p-3",
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", {t!("settings-language")} }
                      div { class: "text-xs text-[var(--secondary-color-5)]", {t!("settings-language-description")} }
                  }
                  LanguagePicker {}
              }
          }
          SettingsSession { title: t!("settings-sync"),
              div { class: "flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  if sync == SyncStatus::Synced {
                      CloudCheck { size: "20px", stroke: sync_icon_color }
                  } else {
                      CloudOff { size: "20px", stroke: sync_icon_color }
                  }
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", "{sync_label}" }
                      div { class: "text-xs text-[var(--secondary-color-5)]", {t!("settings-notes-stored", count: note_count as i64)} }
                  }
                  Button {
                      variant: ButtonVariant::Secondary,
                      size: ButtonSize::Sm,
                      class: "border border-[var(--primary-color-6)] bg-transparent hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]",
                      onclick: move |_| settings.toggle_sync(),
                      if sync == SyncStatus::Offline { {t!("settings-go-online")} } else { {t!("settings-go-offline")} }
                  }
              }
              div { class: "mt-2 flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  HardDrive { size: "20px", stroke: "var(--secondary-color-5)" }
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", {t!("settings-offline-storage")} }
                      div { class: "text-xs text-[var(--secondary-color-5)]", {t!("settings-offline-storage-description")} }
                  }
              }
          }
          SettingsSession { title: t!("settings-editor"),
              div { class: "flex flex-col gap-2 text-sm text-[var(--secondary-color)]",
                  div { class: "flex items-center gap-2",
                      Check { size: "14px", stroke: "var(--accent)" }
                      {t!("settings-editor-markdown")}
                  }
                  div { class: "flex items-center gap-2",
                      Check { size: "14px", stroke: "var(--accent)" }
                      {t!("settings-editor-folders-tags")}
                  }
                  div { class: "flex items-center gap-2",
                      Check { size: "14px", stroke: "var(--accent)" }
                      {t!("settings-editor-search")}
                  }
              }
          }
          SettingsSession { title: t!("settings-about"),
              div { class: "flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  Notebook { size: "20px", stroke: "var(--accent)" }
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", {t!("app-name")} }
                      div { class: "text-xs text-[var(--secondary-color-5)]", {t!("settings-version")} }
                  }
              }
          }
      }
  }
}
