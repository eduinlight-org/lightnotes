use crate::state::use_boot;
use crate::Route;
use dioxus::prelude::*;
use ui::components::sidebar::use_is_mobile;

#[derive(Clone, Copy, PartialEq)]
pub enum Section {
  Notes,
  Diary,
  Settings,
}

fn section_of(route: &Route) -> Section {
  match route {
    Route::Diary {} | Route::DiaryEntry { .. } => Section::Diary,
    Route::Settings {} => Section::Settings,
    _ => Section::Notes,
  }
}

#[derive(Clone, Copy)]
pub struct SectionShellState {
  pub is_mobile: Signal<bool>,
  pub section: Section,
  pub hide_chrome: bool,
  pub viewport_ready: bool,
}

impl SectionShellState {
  pub fn go_to_notes(&self) {
    navigator().push(Route::Notes {});
  }

  pub fn go_to_diary(&self) {
    navigator().push(Route::Diary {});
  }

  pub fn go_to_settings(&self) {
    navigator().push(Route::Settings {});
  }
}

pub fn use_section_shell() -> SectionShellState {
  let is_mobile = use_is_mobile();
  let viewport_ready = (use_boot().viewport_ready)();
  let route = use_route::<Route>();
  let section = section_of(&route);
  let hide_chrome = matches!(route, Route::NoteEditor { .. });

  SectionShellState { is_mobile, section, hide_chrome, viewport_ready }
}
