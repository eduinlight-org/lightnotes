use super::use_settings_panel::use_settings_panel;
use super::SettingsSession;
use crate::state::{SyncStatus, Theme, ACCENT_SWATCHES};
use dioxus::prelude::*;
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

  let (sync_icon_color, sync_label): (&str, &str) = match sync {
    SyncStatus::Synced => ("var(--secondary-color-5)", "All changes saved"),
    SyncStatus::Offline => ("var(--secondary-color-5)", "Offline — saved locally"),
  };

  rsx! {
      div { class: "flex flex-col gap-5",
          SettingsSession { title: "Appearance".to_string(),
              div { class: "flex gap-2",
                  button {
                      class: theme_card_class(theme == Theme::Dark),
                      onclick: move |_| settings.set_theme(Theme::Dark),
                      Moon { size: "20px" }
                      span { class: "text-sm font-medium", "Dark" }
                  }
                  button {
                      class: theme_card_class(theme == Theme::Light),
                      onclick: move |_| settings.set_theme(Theme::Light),
                      Sun { size: "20px" }
                      span { class: "text-sm font-medium", "Light" }
                  }
              }
              div { class: "flex items-center gap-4 rounded-lg bg-[var(--primary-color-3)] p-3",
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", "Accent color" }
                      div { class: "text-xs text-[var(--secondary-color-5)]", "Used for highlights, links and controls" }
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
          SettingsSession { title: "Sync & storage".to_string(),
              div { class: "flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  if sync == SyncStatus::Synced {
                      CloudCheck { size: "20px", stroke: sync_icon_color }
                  } else {
                      CloudOff { size: "20px", stroke: sync_icon_color }
                  }
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", "{sync_label}" }
                      div { class: "text-xs text-[var(--secondary-color-5)]", "{note_count} notes stored locally · offline-ready" }
                  }
                  Button {
                      variant: ButtonVariant::Secondary,
                      size: ButtonSize::Sm,
                      class: "border border-[var(--primary-color-6)] bg-transparent hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]",
                      onclick: move |_| settings.toggle_sync(),
                      if sync == SyncStatus::Offline { "Go online" } else { "Go offline" }
                  }
              }
              div { class: "mt-2 flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  HardDrive { size: "20px", stroke: "var(--secondary-color-5)" }
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", "Offline storage" }
                      div { class: "text-xs text-[var(--secondary-color-5)]", "Stored on this device" }
                  }
              }
          }
          SettingsSession { title: "Editor".to_string(),
              div { class: "flex flex-col gap-2 text-sm text-[var(--secondary-color)]",
                  div { class: "flex items-center gap-2",
                      Check { size: "14px", stroke: "var(--accent)" }
                      "Live Markdown preview"
                  }
                  div { class: "flex items-center gap-2",
                      Check { size: "14px", stroke: "var(--accent)" }
                      "Folders & tags"
                  }
                  div { class: "flex items-center gap-2",
                      Check { size: "14px", stroke: "var(--accent)" }
                      "Full-text local search"
                  }
              }
          }
          SettingsSession { title: "About".to_string(),
              div { class: "flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  Notebook { size: "20px", stroke: "var(--accent)" }
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", "LightNotes" }
                      div { class: "text-xs text-[var(--secondary-color-5)]", "Version 0.1.0 · local-first · multi-platform" }
                  }
              }
          }
      }
  }
}
