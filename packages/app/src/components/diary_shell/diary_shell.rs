use super::use_diary_shell::use_diary_shell;
use crate::components::DiaryEntryList;
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn DiaryShell() -> Element {
  let diary_shell = use_diary_shell();
  let is_mobile = diary_shell.is_mobile;
  let full_page = diary_shell.full_page;

  if is_mobile() {
    return rsx! {
        div { class: "flex h-full flex-col overflow-hidden",
            if full_page {
                Outlet::<Route> {}
            } else {
                div { class: "flex h-full min-h-0 flex-col overflow-hidden",
                    div { class: "flex flex-none items-center p-3",
                        div { class: "flex-1 text-lg font-medium text-[var(--secondary-color)]", {t!("diary-title")} }
                    }
                    DiaryEntryList {}
                }
            }
        }
    };
  }

  rsx! {
      div { class: "flex h-full w-full overflow-hidden",
          aside {
              class: if full_page { "hidden h-full w-[290px] flex-none flex-col overflow-hidden border-r border-[var(--primary-color-6)] bg-[color-mix(in_srgb,var(--primary-color-2)_45%,transparent)] md:flex" } else { "flex h-full w-[290px] flex-none flex-col overflow-hidden border-r border-[var(--primary-color-6)] bg-[color-mix(in_srgb,var(--primary-color-2)_45%,transparent)]" },
              DiaryEntryList {}
          }
          div {
              class: if full_page { "flex h-full flex-1" } else { "hidden h-full flex-1 md:flex" },
              Outlet::<Route> {}
          }
      }
  }
}
