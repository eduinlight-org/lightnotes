mod notes;
pub use notes::{format_relative_time, FolderIcon, Note, NoteFilter, NotesStore, SyncStatus, Theme, ACCENT_SWATCHES};

mod use_notes;
pub use use_notes::use_notes;

mod preferences;

mod sync;
pub use sync::use_synced_notes;

mod ui;
pub use ui::{use_ui, UiState};
