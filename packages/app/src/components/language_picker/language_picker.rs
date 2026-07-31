use super::use_language_picker::use_language_picker;
use crate::components::{ResponsivePopoverContent, ResponsivePopoverRoot, ResponsivePopoverTrigger};
use crate::state::i18n::language_label;
use crate::state::{Language, LANGUAGES};
use dioxus::prelude::*;
use dioxus_i18n::t;
use ui::components::popover::ContentAlign;

const TRIGGER_CLASS: &str = "flex h-9 w-11 flex-none items-center justify-center rounded-md border border-[var(--primary-color-6)] p-0 hover:bg-[color-mix(in_srgb,var(--secondary-color)_6%,transparent)]";

fn flag(language: Language, width: &'static str, height: &'static str) -> Element {
  match language {
    Language::English => rsx! {
        svg {
            width,
            height,
            view_box: "0 0 24 16",
            class: "shrink-0 rounded-[2px]",
            role: "presentation",
            rect { width: "24", height: "16", fill: "#012169" }
            path { d: "M0,0 L24,16 M24,0 L0,16", stroke: "#ffffff", stroke_width: "3.2" }
            path { d: "M0,0 L24,16 M24,0 L0,16", stroke: "#c8102e", stroke_width: "1.8" }
            path { d: "M12,0 V16 M0,8 H24", stroke: "#ffffff", stroke_width: "5.4" }
            path { d: "M12,0 V16 M0,8 H24", stroke: "#c8102e", stroke_width: "3.2" }
        }
    },
    Language::Spanish => rsx! {
        svg {
            width,
            height,
            view_box: "0 0 24 16",
            class: "shrink-0 rounded-[2px]",
            role: "presentation",
            rect { width: "24", height: "16", fill: "#aa151b" }
            rect { y: "4", width: "24", height: "8", fill: "#f1bf00" }
        }
    },
  }
}

fn option_class(active: bool, mobile: bool) -> &'static str {
  match (active, mobile) {
    (true, true) => "flex cursor-pointer items-center gap-3 rounded-[11px] bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] px-[14px] py-[13px] text-[15px] text-[var(--accent)]",
    (false, true) => "flex cursor-pointer items-center gap-3 rounded-[11px] bg-[var(--primary-color-2)] px-[14px] py-[13px] text-[15px] text-[var(--secondary-color)]",
    (true, false) => "flex cursor-pointer items-center gap-2.5 rounded-md bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] px-2 py-1.5 text-sm text-[var(--accent)]",
    (false, false) => "flex cursor-pointer items-center gap-2.5 rounded-md px-2 py-1.5 text-sm text-[var(--secondary-color)] hover:bg-[var(--primary-color-4)]",
  }
}

#[component]
pub fn LanguagePicker() -> Element {
  let mut picker = use_language_picker();
  let mobile = (picker.is_mobile)();
  let language = picker.language();

  rsx! {
      ResponsivePopoverRoot {
          open: (picker.open)(),
          on_open_change: move |value| picker.set_open(value),
          ResponsivePopoverTrigger {
              class: TRIGGER_CLASS,
              "aria-label": t!("settings-language"),
              title: language_label(language),
              {flag(language, "24", "16")}
          }
          ResponsivePopoverContent {
              title: t!("settings-language"),
              align: ContentAlign::End,
              class: "w-44 items-stretch gap-1 p-1.5 text-left",
              for option in LANGUAGES {
                  div {
                      key: "{option.code()}",
                      class: option_class(option == language, mobile),
                      onclick: move |_| picker.select(option),
                      {flag(option, "22", "15")}
                      span { class: "flex-1", {language_label(option)} }
                  }
              }
          }
      }
  }
}
