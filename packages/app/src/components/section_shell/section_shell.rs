use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Calendar, Notebook, Settings as SettingsIcon};
use ui::components::sidebar::use_is_mobile;

#[derive(Clone, Copy, PartialEq)]
enum Section {
  Notes,
  Diary,
  Settings,
}

fn section_of(route: &Route) -> Section {
  match route {
    Route::Diary {} => Section::Diary,
    Route::Settings {} => Section::Settings,
    _ => Section::Notes,
  }
}

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
  let is_mobile = use_is_mobile();
  let route = use_route::<Route>();
  let section = section_of(&route);
  let hide_chrome = matches!(route, Route::NoteEditor { .. });

  if is_mobile() {
    return rsx! {
        div { class: "flex h-full flex-col overflow-hidden",
            div { class: "min-h-0 flex-1 overflow-hidden", Outlet::<Route> {} }
            if !hide_chrome {
                div { class: "flex flex-none border-t border-[var(--primary-color-6)] py-1",
                    button {
                        class: tab_button_class(section == Section::Notes),
                        onclick: move |_| {
                            navigator().push(Route::Notes {});
                        },
                        Notebook { size: "22px" }
                        "Notes"
                    }
                    button {
                        class: tab_button_class(section == Section::Diary),
                        onclick: move |_| {
                            navigator().push(Route::Diary {});
                        },
                        Calendar { size: "22px" }
                        "Diary"
                    }
                    button {
                        class: tab_button_class(section == Section::Settings),
                        onclick: move |_| {
                            navigator().push(Route::Settings {});
                        },
                        SettingsIcon { size: "22px" }
                        "Settings"
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
                  onclick: move |_| {
                      navigator().push(Route::Notes {});
                  },
                  Notebook { size: "21px" }
                  "Notes"
              }
              button {
                  class: rail_button_class(section == Section::Diary),
                  onclick: move |_| {
                      navigator().push(Route::Diary {});
                  },
                  Calendar { size: "21px" }
                  "Diary"
              }
              button {
                  class: "{rail_button_class(section == Section::Settings)} mt-auto",
                  onclick: move |_| {
                      navigator().push(Route::Settings {});
                  },
                  SettingsIcon { size: "21px" }
                  "Settings"
              }
          }
          div {
              class: "min-h-0 flex-1 overflow-hidden [transform:translateZ(0)]",
              Outlet::<Route> {}
          }
      }
  }
}
