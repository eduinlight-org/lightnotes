use super::use_section_shell::{use_section_shell, Section};
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_icons::lucide::{Calendar, Notebook, Settings as SettingsIcon};

fn rail_button_class(active: bool) -> &'static str {
  if active {
    "flex w-[54px] flex-none flex-col items-center gap-1 rounded-lg px-1 py-2 text-[10.5px] font-medium bg-[color-mix(in_srgb,var(--accent)_16%,transparent)] text-[var(--accent)]"
  } else {
    "flex w-[54px] flex-none flex-col items-center gap-1 rounded-lg px-1 py-2 text-[10.5px] font-medium text-[var(--secondary-color-5)] hover:bg-[var(--primary-color-4)]"
  }
}

fn tab_button_class(active: bool) -> &'static str {
  if active {
    "flex flex-1 flex-col items-center gap-1 py-1 text-[11px] text-[var(--accent)]"
  } else {
    "flex flex-1 flex-col items-center gap-1 py-1 text-[11px] text-[var(--secondary-color-5)]"
  }
}

#[component]
pub fn SectionShell() -> Element {
  let section_shell = use_section_shell();
  let is_mobile = section_shell.is_mobile;
  let section = section_shell.section;
  let hide_chrome = section_shell.hide_chrome;

  if !section_shell.viewport_ready {
    return rsx! {
        div { class: "h-full w-full" }
    };
  }

  if is_mobile() {
    return rsx! {
        div { class: "flex h-full flex-col overflow-hidden",
            div { class: "min-h-0 flex-1 overflow-hidden", Outlet::<Route> {} }
            if !hide_chrome {
                div { class: "flex flex-none border-t border-[var(--primary-color-6)] py-1",
                    button {
                        class: tab_button_class(section == Section::Notes),
                        onclick: move |_| section_shell.go_to_notes(),
                        Notebook { size: "22px" }
                        {t!("section-notes")}
                    }
                    button {
                        class: tab_button_class(section == Section::Diary),
                        onclick: move |_| section_shell.go_to_diary(),
                        Calendar { size: "22px" }
                        {t!("section-diary")}
                    }
                    button {
                        class: tab_button_class(section == Section::Settings),
                        onclick: move |_| section_shell.go_to_settings(),
                        SettingsIcon { size: "22px" }
                        {t!("section-settings")}
                    }
                }
            }
        }
    };
  }

  rsx! {
      div { class: "flex h-full w-full overflow-hidden",
          nav { class: "flex w-[66px] flex-none flex-col items-center gap-1 border-r border-[var(--primary-color-6)] bg-[var(--primary-color-2)] p-1.5",
              button {
                  class: rail_button_class(section == Section::Notes),
                  onclick: move |_| section_shell.go_to_notes(),
                  Notebook { size: "21px" }
                  {t!("section-notes")}
              }
              button {
                  class: rail_button_class(section == Section::Diary),
                  onclick: move |_| section_shell.go_to_diary(),
                  Calendar { size: "21px" }
                  {t!("section-diary")}
              }
              button {
                  class: "{rail_button_class(section == Section::Settings)} mt-auto",
                  onclick: move |_| section_shell.go_to_settings(),
                  SettingsIcon { size: "21px" }
                  {t!("section-settings")}
              }
          }
          div {
              class: "min-h-0 flex-1 overflow-hidden [transform:translateZ(0)]",
              Outlet::<Route> {}
          }
      }
  }
}
