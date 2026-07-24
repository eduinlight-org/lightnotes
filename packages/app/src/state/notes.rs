use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
  pub id: String,
  pub title: String,
  pub content: String,
  pub folder_id: Option<String>,
  pub tag_ids: Vec<String>,
  pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Folder {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoteFilter {
  All,
  Folder(String),
  Tag(String),
}

#[derive(Clone, Copy)]
pub struct NotesStore {
  notes: Signal<Vec<Note>>,
  folders: Signal<Vec<Folder>>,
  tags: Signal<Vec<Tag>>,
  filter: Signal<NoteFilter>,
  search: Signal<String>,
  next_id: Signal<u32>,
}

impl NotesStore {
  pub fn seed() -> Self {
    let folders = vec![
      Folder { id: "folder-1".into(), name: "Personal".into() },
      Folder { id: "folder-2".into(), name: "Work".into() },
      Folder { id: "folder-3".into(), name: "Ideas".into() },
    ];

    let tags = vec![
      Tag { id: "tag-1".into(), name: "todo".into() },
      Tag { id: "tag-2".into(), name: "recipe".into() },
      Tag { id: "tag-3".into(), name: "journal".into() },
    ];

    // Kept in most-recent-first order; `create_note` inserts new notes at the
    // front so the store never needs to derive an ordering from `updated_at`.
    let notes = vec![
      Note {
        id: "note-1".into(),
        title: "Welcome to Notes".into(),
        content: "# Welcome\n\nThis is your first note. Try **Markdown** formatting, add tags, and organize notes into folders.\n\n- Write freely\n- Use the toolbar for formatting\n- Search from the top bar".into(),
        folder_id: Some("folder-3".into()),
        tag_ids: vec!["tag-3".into()],
        updated_at: "2h ago".into(),
      },
      Note {
        id: "note-2".into(),
        title: "Grocery list".into(),
        content: "## Groceries\n\n- Milk\n- Eggs\n- Sourdough bread\n- Coffee".into(),
        folder_id: Some("folder-1".into()),
        tag_ids: vec!["tag-1".into()],
        updated_at: "Yesterday".into(),
      },
      Note {
        id: "note-3".into(),
        title: "Sourdough recipe".into(),
        content: "# Sourdough Bread\n\n1. Mix flour and water\n2. Add starter\n3. Fold every 30 minutes\n4. Bake at 230C for 40 minutes".into(),
        folder_id: Some("folder-1".into()),
        tag_ids: vec!["tag-2".into()],
        updated_at: "2 days ago".into(),
      },
      Note {
        id: "note-4".into(),
        title: "Q3 roadmap notes".into(),
        content: "# Q3 Roadmap\n\n- Ship offline storage\n- Draft cloud sync design\n- Review onboarding flow".into(),
        folder_id: Some("folder-2".into()),
        tag_ids: vec!["tag-1".into()],
        updated_at: "3 days ago".into(),
      },
      Note {
        id: "note-5".into(),
        title: "Morning pages".into(),
        content: "Started the day with a walk. Feeling good about the new notes app design.".into(),
        folder_id: None,
        tag_ids: vec!["tag-3".into()],
        updated_at: "1 week ago".into(),
      },
    ];

    Self {
      notes: Signal::new(notes),
      folders: Signal::new(folders),
      tags: Signal::new(tags),
      filter: Signal::new(NoteFilter::All),
      search: Signal::new(String::new()),
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

  pub fn search(&self) -> String {
    (self.search)()
  }

  pub fn note(&self, id: &str) -> Option<Note> {
    (self.notes)().into_iter().find(|note| note.id == id)
  }

  pub fn visible_notes(&self) -> Vec<Note> {
    let filter = self.filter();
    let query = self.search().to_lowercase();

    (self.notes)()
      .into_iter()
      .filter(|note| match &filter {
        NoteFilter::All => true,
        NoteFilter::Folder(folder_id) => note.folder_id.as_deref() == Some(folder_id.as_str()),
        NoteFilter::Tag(tag_id) => note.tag_ids.iter().any(|id| id == tag_id),
      })
      .filter(|note| {
        query.is_empty()
          || note.title.to_lowercase().contains(&query)
          || note.content.to_lowercase().contains(&query)
      })
      .collect()
  }

  pub fn set_filter(&mut self, filter: NoteFilter) {
    self.filter.set(filter);
  }

  pub fn set_search(&mut self, query: String) {
    self.search.set(query);
  }

  pub fn create_note(&mut self) -> String {
    let id = format!("note-{}", self.next_id());

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
        updated_at: "Just now".into(),
      },
    );

    id
  }

  pub fn set_note_title(&mut self, id: &str, title: String) {
    if let Some(note) = self.notes.write().iter_mut().find(|note| note.id == id) {
      note.title = title;
      note.updated_at = "Just now".into();
    }
  }

  pub fn set_note_content(&mut self, id: &str, content: String) {
    if let Some(note) = self.notes.write().iter_mut().find(|note| note.id == id) {
      note.content = content;
      note.updated_at = "Just now".into();
    }
  }

  pub fn add_note_tag(&mut self, id: &str, tag_id: String) {
    if let Some(note) = self.notes.write().iter_mut().find(|note| note.id == id) {
      if !note.tag_ids.contains(&tag_id) {
        note.tag_ids.push(tag_id);
      }
      note.updated_at = "Just now".into();
    }
  }

  pub fn remove_note_tag(&mut self, id: &str, tag_id: &str) {
    if let Some(note) = self.notes.write().iter_mut().find(|note| note.id == id) {
      note.tag_ids.retain(|existing| existing != tag_id);
      note.updated_at = "Just now".into();
    }
  }

  /// Finds a tag by (case-insensitive) name, creating it if it doesn't exist yet.
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

  pub fn create_folder(&mut self, name: String) -> String {
    let id = format!("folder-{}", self.next_id());
    self.folders.write().push(Folder { id: id.clone(), name });
    id
  }

  pub fn create_tag(&mut self, name: String) -> String {
    let id = format!("tag-{}", self.next_id());
    self.tags.write().push(Tag { id: id.clone(), name });
    id
  }
}
