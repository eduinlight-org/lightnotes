use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub const ACCENT_SWATCHES: [&str; 6] = ["#9184d9", "#84a7d9", "#7db8a0", "#d99184", "#c9a24b", "#c58fd0"];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
  pub id: String,
  pub title: String,
  pub content: String,
  pub folder_id: Option<String>,
  pub tag_ids: Vec<String>,
  pub pinned: bool,
  pub starred: bool,
  pub updated_at: String,
  pub order: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FolderIcon {
  Inbox,
  Briefcase,
  User,
  BookOpen,
  Notebook,
  Archive,
  House,
  Star,
  Heart,
  Settings,
  Calendar,
  Camera,
  Music,
  Code,
  Palette,
  Gift,
  Globe,
  Lock,
  Rocket,
  Bookmark,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Folder {
  pub id: String,
  pub name: String,
  pub icon: FolderIcon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum Theme {
  #[default]
  Dark,
  Light,
}

impl Theme {
  pub fn as_str(&self) -> &'static str {
    match self {
      Theme::Dark => "dark",
      Theme::Light => "light",
    }
  }

}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum SyncStatus {
  #[default]
  Synced,
  Offline,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoteFilter {
  All,
  Starred,
  Pinned,
  Folder(String),
  Tag(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedState {
  pub notes: Vec<Note>,
  pub folders: Vec<Folder>,
  pub tags: Vec<Tag>,
  pub theme: Theme,
  #[serde(default = "default_accent")]
  pub accent: String,
  pub sync: SyncStatus,
  pub next_id: u32,
}

fn default_accent() -> String {
  ACCENT_SWATCHES[0].to_string()
}

#[derive(Clone, Copy)]
pub struct NotesStore {
  notes: Signal<Vec<Note>>,
  folders: Signal<Vec<Folder>>,
  tags: Signal<Vec<Tag>>,
  filter: Signal<NoteFilter>,
  search: Signal<String>,
  theme: Signal<Theme>,
  accent: Signal<String>,
  sync: Signal<SyncStatus>,
  next_id: Signal<u32>,
}

impl NotesStore {
  pub fn seed() -> Self {
    let folders = vec![
      Folder { id: "folder-1".into(), name: "Personal".into(), icon: FolderIcon::User },
      Folder { id: "folder-2".into(), name: "Work".into(), icon: FolderIcon::Briefcase },
      Folder { id: "folder-3".into(), name: "Ideas".into(), icon: FolderIcon::Inbox },
    ];

    let tags = vec![
      Tag { id: "tag-1".into(), name: "todo".into() },
      Tag { id: "tag-2".into(), name: "recipe".into() },
      Tag { id: "tag-3".into(), name: "journal".into() },
    ];

    let notes = vec![
      Note {
        id: "note-1".into(),
        title: "Welcome to LightNotes".into(),
        content: "# Welcome\n\nThis is your first note. Try **Markdown** formatting, add tags, and organize notes into folders.\n\n- Write freely\n- Use the toolbar for formatting\n- Search from the top bar\n\n## Getting started\n- [x] Read this note\n- [ ] Create your first note\n- [ ] Star your favorite\n\n> Local-first: everything is saved on this device.".into(),
        folder_id: Some("folder-3".into()),
        tag_ids: vec!["tag-3".into()],
        pinned: true,
        starred: true,
        updated_at: "2h ago".into(),
        order: 100,
      },
      Note {
        id: "note-2".into(),
        title: "Grocery list".into(),
        content: "## Groceries\n\n- Milk\n- Eggs\n- Sourdough bread\n- Coffee".into(),
        folder_id: Some("folder-1".into()),
        tag_ids: vec!["tag-1".into()],
        pinned: false,
        starred: false,
        updated_at: "Yesterday".into(),
        order: 90,
      },
      Note {
        id: "note-3".into(),
        title: "Sourdough recipe".into(),
        content: "# Sourdough Bread\n\n1. Mix flour and water\n2. Add starter\n3. Fold every 30 minutes\n4. Bake at 230C for 40 minutes".into(),
        folder_id: Some("folder-1".into()),
        tag_ids: vec!["tag-2".into()],
        pinned: false,
        starred: false,
        updated_at: "2 days ago".into(),
        order: 80,
      },
      Note {
        id: "note-4".into(),
        title: "Q3 roadmap notes".into(),
        content: "# Q3 Roadmap\n\n- Ship offline storage\n- Draft cloud sync design\n- Review onboarding flow".into(),
        folder_id: Some("folder-2".into()),
        tag_ids: vec!["tag-1".into()],
        pinned: true,
        starred: false,
        updated_at: "3 days ago".into(),
        order: 70,
      },
      Note {
        id: "note-5".into(),
        title: "Morning pages".into(),
        content: "Started the day with a walk. Feeling good about the new notes app design.".into(),
        folder_id: None,
        tag_ids: vec!["tag-3".into()],
        pinned: false,
        starred: false,
        updated_at: "1 week ago".into(),
        order: 60,
      },
    ];

    Self {
      notes: Signal::new(notes),
      folders: Signal::new(folders),
      tags: Signal::new(tags),
      filter: Signal::new(NoteFilter::All),
      search: Signal::new(String::new()),
      theme: Signal::new(Theme::Dark),
      accent: Signal::new(ACCENT_SWATCHES[0].to_string()),
      sync: Signal::new(SyncStatus::Synced),
      next_id: Signal::new(6),
    }
  }

  fn next_id(&mut self) -> u32 {
    let id = (self.next_id)();
    self.next_id.set(id + 1);
    id
  }

  pub fn folders(&self) -> Vec<Folder> {
    (self.folders)()
  }

  pub fn tags(&self) -> Vec<Tag> {
    (self.tags)()
  }

  pub fn tag_name(&self, id: &str) -> Option<String> {
    (self.tags)().into_iter().find(|tag| tag.id == id).map(|tag| tag.name)
  }

  pub fn filter(&self) -> NoteFilter {
    (self.filter)()
  }

  pub fn filter_title(&self) -> String {
    match self.filter() {
      NoteFilter::All => "All Notes".to_string(),
      NoteFilter::Starred => "Starred".to_string(),
      NoteFilter::Pinned => "Pinned".to_string(),
      NoteFilter::Tag(tag_id) => self
        .tag_name(&tag_id)
        .map(|name| format!("#{name}"))
        .unwrap_or_default(),
      NoteFilter::Folder(folder_id) => self
        .folders()
        .into_iter()
        .find(|folder| folder.id == folder_id)
        .map(|folder| folder.name)
        .unwrap_or_default(),
    }
  }

  pub fn search(&self) -> String {
    (self.search)()
  }

  pub fn theme(&self) -> Theme {
    (self.theme)()
  }

  pub fn accent(&self) -> String {
    (self.accent)()
  }

  pub fn sync(&self) -> SyncStatus {
    (self.sync)()
  }

  pub fn note(&self, id: &str) -> Option<Note> {
    (self.notes)().into_iter().find(|note| note.id == id)
  }

  pub fn note_count(&self) -> usize {
    (self.notes)().len()
  }

  pub fn starred_count(&self) -> usize {
    (self.notes)().iter().filter(|note| note.starred).count()
  }

  pub fn pinned_count(&self) -> usize {
    (self.notes)().iter().filter(|note| note.pinned).count()
  }

  pub fn folder_note_count(&self, folder_id: &str) -> usize {
    (self.notes)()
      .iter()
      .filter(|note| note.folder_id.as_deref() == Some(folder_id))
      .count()
  }

  pub fn tag_note_count(&self, tag_id: &str) -> usize {
    (self.notes)()
      .iter()
      .filter(|note| note.tag_ids.iter().any(|id| id == tag_id))
      .count()
  }

  pub fn visible_notes(&self) -> Vec<Note> {
    let filter = self.filter();
    let query = self.search().to_lowercase();

    let mut notes: Vec<Note> = (self.notes)()
      .into_iter()
      .filter(|note| match &filter {
        NoteFilter::All => true,
        NoteFilter::Starred => note.starred,
        NoteFilter::Pinned => note.pinned,
        NoteFilter::Folder(folder_id) => note.folder_id.as_deref() == Some(folder_id.as_str()),
        NoteFilter::Tag(tag_id) => note.tag_ids.iter().any(|id| id == tag_id),
      })
      .filter(|note| {
        query.is_empty()
          || note.title.to_lowercase().contains(&query)
          || note.content.to_lowercase().contains(&query)
      })
      .collect();

    notes.sort_by(|a, b| b.pinned.cmp(&a.pinned).then(b.order.cmp(&a.order)));
    notes
  }

  pub fn set_filter(&mut self, filter: NoteFilter) {
    self.filter.set(filter);
    self.search.set(String::new());
  }

  pub fn set_search(&mut self, query: String) {
    self.search.set(query);
  }

  pub fn set_theme(&mut self, theme: Theme) {
    self.theme.set(theme);
  }

  pub fn set_accent(&mut self, accent: String) {
    self.accent.set(accent);
  }

  pub fn toggle_sync(&mut self) {
    let next = match self.sync() {
      SyncStatus::Synced => SyncStatus::Offline,
      SyncStatus::Offline => SyncStatus::Synced,
    };
    self.sync.set(next);
  }

  pub fn create_note(&mut self) -> String {
    let order = self.next_id() as i64;
    let id = format!("note-{order}");

    let folder_id = match self.filter() {
      NoteFilter::Folder(folder_id) => Some(folder_id),
      _ => None,
    };
    let tag_ids = match self.filter() {
      NoteFilter::Tag(tag_id) => vec![tag_id],
      _ => Vec::new(),
    };

    self.notes.write().insert(
      0,
      Note {
        id: id.clone(),
        title: "Untitled note".into(),
        content: String::new(),
        folder_id,
        tag_ids,
        pinned: false,
        starred: false,
        updated_at: "Just now".into(),
        order,
      },
    );

    id
  }

  fn touch_note(&mut self, id: &str) {
    let order = self.next_id() as i64;
    if let Some(note) = self.notes.write().iter_mut().find(|note| note.id == id) {
      note.updated_at = "Just now".into();
      note.order = order;
    }
  }

  pub fn set_note_title(&mut self, id: &str, title: String) {
    if let Some(note) = self.notes.write().iter_mut().find(|note| note.id == id) {
      note.title = title;
    }
    self.touch_note(id);
  }

  pub fn set_note_content(&mut self, id: &str, content: String) {
    if let Some(note) = self.notes.write().iter_mut().find(|note| note.id == id) {
      note.content = content;
    }
    self.touch_note(id);
  }

  pub fn toggle_note_pin(&mut self, id: &str) {
    if let Some(note) = self.notes.write().iter_mut().find(|note| note.id == id) {
      note.pinned = !note.pinned;
    }
  }

  pub fn toggle_note_star(&mut self, id: &str) {
    if let Some(note) = self.notes.write().iter_mut().find(|note| note.id == id) {
      note.starred = !note.starred;
    }
  }

  pub fn set_note_folder(&mut self, id: &str, folder_id: Option<String>) {
    if let Some(note) = self.notes.write().iter_mut().find(|note| note.id == id) {
      note.folder_id = folder_id;
    }
    self.touch_note(id);
  }

  pub fn add_note_tag(&mut self, id: &str, tag_id: String) {
    if let Some(note) = self.notes.write().iter_mut().find(|note| note.id == id) {
      if !note.tag_ids.contains(&tag_id) {
        note.tag_ids.push(tag_id);
      }
    }
    self.touch_note(id);
  }

  pub fn remove_note_tag(&mut self, id: &str, tag_id: &str) {
    if let Some(note) = self.notes.write().iter_mut().find(|note| note.id == id) {
      note.tag_ids.retain(|existing| existing != tag_id);
    }
    self.touch_note(id);
  }

  pub fn tag_id_for_name(&mut self, name: &str) -> String {
    let existing = (self.tags)()
      .into_iter()
      .find(|tag| tag.name.eq_ignore_ascii_case(name))
      .map(|tag| tag.id);

    existing.unwrap_or_else(|| self.create_tag(name.to_string()))
  }

  pub fn delete_note(&mut self, id: &str) {
    self.notes.write().retain(|note| note.id != id);
  }

  pub fn create_folder_with_icon(&mut self, name: String, icon: FolderIcon) -> String {
    let id = format!("folder-{}", self.next_id());
    self.folders.write().push(Folder { id: id.clone(), name, icon });
    id
  }

  pub fn rename_folder(&mut self, folder_id: &str, name: String) {
    if let Some(folder) = self.folders.write().iter_mut().find(|folder| folder.id == folder_id) {
      folder.name = name;
    }
  }

  pub fn set_folder_icon(&mut self, folder_id: &str, icon: FolderIcon) {
    if let Some(folder) = self.folders.write().iter_mut().find(|folder| folder.id == folder_id) {
      folder.icon = icon;
    }
  }

  pub fn delete_folder(&mut self, folder_id: &str) {
    self.folders.write().retain(|folder| folder.id != folder_id);
    for note in self.notes.write().iter_mut() {
      if note.folder_id.as_deref() == Some(folder_id) {
        note.folder_id = None;
      }
    }
    if self.filter() == NoteFilter::Folder(folder_id.to_string()) {
      self.set_filter(NoteFilter::All);
    }
  }

  pub fn create_tag(&mut self, name: String) -> String {
    let normalized = name.trim().to_lowercase().replace(' ', "-");
    if let Some(existing) = (self.tags)().into_iter().find(|tag| tag.name == normalized) {
      return existing.id;
    }
    let id = format!("tag-{}", self.next_id());
    self.tags.write().push(Tag { id: id.clone(), name: normalized });
    id
  }

  pub fn delete_tag(&mut self, tag_id: &str) {
    self.tags.write().retain(|tag| tag.id != tag_id);
    for note in self.notes.write().iter_mut() {
      note.tag_ids.retain(|id| id != tag_id);
    }
    if self.filter() == NoteFilter::Tag(tag_id.to_string()) {
      self.set_filter(NoteFilter::All);
    }
  }

  pub fn snapshot(&self) -> PersistedState {
    PersistedState {
      notes: (self.notes)(),
      folders: (self.folders)(),
      tags: (self.tags)(),
      theme: self.theme(),
      accent: self.accent(),
      sync: self.sync(),
      next_id: (self.next_id)(),
    }
  }

  pub fn restore(&mut self, state: PersistedState) {
    self.notes.set(state.notes);
    self.folders.set(state.folders);
    self.tags.set(state.tags);
    self.theme.set(state.theme);
    self.accent.set(state.accent);
    self.sync.set(state.sync);
    self.next_id.set(state.next_id);
  }
}
