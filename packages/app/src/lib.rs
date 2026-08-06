//! This crate contains the shared routes, pages and layout for the workspace.

use dioxus::prelude::*;

pub mod boot;

mod components;
use components::{AppShell, DiaryShell, NotesShell, RootShell, SectionShell};

mod state;

mod views;
pub use views::{Diary, DiaryEntry, FoldersScreen, Login, NoteEditor, Notes, Settings, TagsScreen};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(RootShell)]
      #[route("/login")]
      Login {},
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
          #[layout(DiaryShell)]
            #[route("/diary")]
            Diary {},
            #[route("/diary/:note_id")]
            DiaryEntry { note_id: String },
          #[end_layout]
          #[route("/settings")]
          Settings {},
}
