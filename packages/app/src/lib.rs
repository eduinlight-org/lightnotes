//! This crate contains the shared routes, pages and layout for the workspace.

use dioxus::prelude::*;

mod components;
use components::{AppShell, NotesShell, SectionShell};

mod state;

mod views;
pub use views::{Diary, FoldersScreen, NoteEditor, Notes, Settings, TagsScreen};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppShell)]
      #[layout(SectionShell)]
        #[layout(NotesShell)]
          #[route("/")]
          Notes {},
          #[route("/note/:note_id")]
          NoteEditor { note_id: String },
          #[route("/tags")]
          TagsScreen {},
          #[route("/folders")]
          FoldersScreen {},
        #[end_layout]
        #[route("/diary")]
        Diary {},
        #[route("/settings")]
        Settings {},
}
