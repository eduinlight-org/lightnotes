//! This crate contains the shared routes, pages and layout for the workspace.

use dioxus::prelude::*;

mod components;
use components::{AppShell, NotesShell};

mod state;

mod views;
pub use views::{NoteEditor, Notes};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppShell)]
      #[layout(NotesShell)]
        #[route("/")]
        Notes {},
        #[route("/note/:note_id")]
        NoteEditor { note_id: String },
}
