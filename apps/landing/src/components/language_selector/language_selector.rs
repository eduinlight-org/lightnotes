use super::use_language_selector::use_language_selector;
use crate::i18n::{Language, LANGUAGES};
use dioxus::prelude::*;
use dioxus_i18n::t;

const TRIGGER_CLASS: &str = "flex h-9 w-11 cursor-pointer items-center justify-center rounded-md border border-divider bg-transparent hover:border-accent";
const MENU_CLASS: &str = "absolute right-0 top-[calc(100%+8px)] z-50 flex min-w-[176px] flex-col gap-0.5 rounded-md border border-divider bg-surface p-1.5 shadow-md";
const SHEET_CLASS: &str = "fixed inset-x-0 bottom-0 z-50 flex flex-col gap-1.5 rounded-t-lg border-t border-divider bg-surface p-3 pb-[calc(18px+env(safe-area-inset-bottom))] shadow-lg";
const SHEET_TITLE_CLASS: &str = "px-2 pb-1 text-[12px] uppercase tracking-[0.06em] text-copy-ghost";

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
    (true, true) => "flex cursor-pointer items-center gap-3 rounded-md bg-[color-mix(in_srgb,var(--color-accent)_16%,transparent)] px-3 py-3 text-[15px] text-accent",
    (false, true) => "flex cursor-pointer items-center gap-3 rounded-md px-3 py-3 text-[15px] text-copy",
    (true, false) => "flex cursor-pointer items-center gap-2.5 rounded-sm bg-[color-mix(in_srgb,var(--color-accent)_16%,transparent)] px-2.5 py-1.5 text-[14px] text-accent",
    (false, false) => "flex cursor-pointer items-center gap-2.5 rounded-sm px-2.5 py-1.5 text-[14px] text-copy hover:text-accent",
  }
}

fn language_label(language: Language) -> String {
  t!(language.label_key())
}

#[component]
pub fn LanguageSelector() -> Element {
  let mut selector = use_language_selector();
  let open = (selector.open)();
  let mobile = (selector.is_mobile)();

  rsx! {
      div {
          class: "relative flex items-center",
          button {
              class: TRIGGER_CLASS,
              "aria-label": t!("nav-language"),
              "aria-haspopup": "menu",
              "aria-expanded": open,
              title: language_label(selector.current),
              onclick: move |_| selector.toggle(),
              {flag(selector.current, "24", "16")}
          }
          if open {
              div {
                  class: if mobile { "fixed inset-0 z-40 bg-black/55" } else { "fixed inset-0 z-40" },
                  onclick: move |_| selector.close(),
              }
              div {
                  class: if mobile { SHEET_CLASS } else { MENU_CLASS },
                  role: "menu",
                  if mobile {
                      span { class: SHEET_TITLE_CLASS, {t!("nav-language")} }
                  }
                  for language in LANGUAGES {
                      button {
                          key: "{language.code()}",
                          class: option_class(language == selector.current, mobile),
                          role: "menuitem",
                          "aria-current": language == selector.current,
                          lang: language.code(),
                          onclick: move |_| selector.select(language),
                          {flag(language, "22", "15")}
                          span { class: "flex-1 text-left", {language_label(language)} }
                      }
                  }
              }
          }
      }
  }
}
