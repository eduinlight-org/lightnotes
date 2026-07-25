mod notes;
pub use notes::{FolderIcon, Note, NoteFilter, NotesStore, SyncStatus, Theme, ACCENT_SWATCHES};

mod use_notes;
pub use use_notes::use_notes;

mod storage;
pub use storage::use_persisted_notes;

mod ui;
pub use ui::{use_ui, UiState};
