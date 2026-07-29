use std::collections::{HashMap, HashSet};

use sync_dto::{ChangeOp, ChangePayload, EntityKind, FolderDto, FolderIconDto, NoteDto, QueuedChange, ServerChange, TagDto};

use crate::state::notes::{now_ms, Folder, FolderIcon, Note, PersistedState, Tag};

pub const DEFAULT_API_BASE_URL: &str = "http://localhost:4000";

pub fn api_base_url() -> String {
  std::env::var("API_BASE_URL").unwrap_or_else(|_| DEFAULT_API_BASE_URL.to_string())
}

pub fn folder_icon_to_dto(icon: FolderIcon) -> FolderIconDto {
  match icon {
    FolderIcon::Inbox => FolderIconDto::Inbox,
    FolderIcon::Briefcase => FolderIconDto::Briefcase,
    FolderIcon::User => FolderIconDto::User,
    FolderIcon::BookOpen => FolderIconDto::BookOpen,
    FolderIcon::Notebook => FolderIconDto::Notebook,
    FolderIcon::Archive => FolderIconDto::Archive,
    FolderIcon::House => FolderIconDto::House,
    FolderIcon::Star => FolderIconDto::Star,
    FolderIcon::Heart => FolderIconDto::Heart,
    FolderIcon::Settings => FolderIconDto::Settings,
    FolderIcon::Calendar => FolderIconDto::Calendar,
    FolderIcon::Camera => FolderIconDto::Camera,
    FolderIcon::Music => FolderIconDto::Music,
    FolderIcon::Code => FolderIconDto::Code,
    FolderIcon::Palette => FolderIconDto::Palette,
    FolderIcon::Gift => FolderIconDto::Gift,
    FolderIcon::Globe => FolderIconDto::Globe,
    FolderIcon::Lock => FolderIconDto::Lock,
    FolderIcon::Rocket => FolderIconDto::Rocket,
    FolderIcon::Bookmark => FolderIconDto::Bookmark,
  }
}

pub fn folder_icon_from_dto(icon: FolderIconDto) -> FolderIcon {
  match icon {
    FolderIconDto::Inbox => FolderIcon::Inbox,
    FolderIconDto::Briefcase => FolderIcon::Briefcase,
    FolderIconDto::User => FolderIcon::User,
    FolderIconDto::BookOpen => FolderIcon::BookOpen,
    FolderIconDto::Notebook => FolderIcon::Notebook,
    FolderIconDto::Archive => FolderIcon::Archive,
    FolderIconDto::House => FolderIcon::House,
    FolderIconDto::Star => FolderIcon::Star,
    FolderIconDto::Heart => FolderIcon::Heart,
    FolderIconDto::Settings => FolderIcon::Settings,
    FolderIconDto::Calendar => FolderIcon::Calendar,
    FolderIconDto::Camera => FolderIcon::Camera,
    FolderIconDto::Music => FolderIcon::Music,
    FolderIconDto::Code => FolderIcon::Code,
    FolderIconDto::Palette => FolderIcon::Palette,
    FolderIconDto::Gift => FolderIcon::Gift,
    FolderIconDto::Globe => FolderIcon::Globe,
    FolderIconDto::Lock => FolderIcon::Lock,
    FolderIconDto::Rocket => FolderIcon::Rocket,
    FolderIconDto::Bookmark => FolderIcon::Bookmark,
  }
}

pub fn note_to_dto(note: &Note) -> NoteDto {
  NoteDto {
    id: note.id.clone(),
    title: note.title.clone(),
    content: note.content.clone(),
    folder_id: note.folder_id.clone(),
    tag_ids: note.tag_ids.clone(),
    pinned: note.pinned,
    starred: note.starred,
    updated_at_ms: note.updated_at_ms,
    order: note.order,
    date_ms: note.date_ms,
    remind_before_hours: note.remind_before_hours,
  }
}

pub fn note_from_dto(dto: NoteDto) -> Note {
  Note {
    id: dto.id,
    title: dto.title,
    content: dto.content,
    folder_id: dto.folder_id,
    tag_ids: dto.tag_ids,
    pinned: dto.pinned,
    starred: dto.starred,
    updated_at_ms: dto.updated_at_ms,
    order: dto.order,
    date_ms: dto.date_ms,
    remind_before_hours: dto.remind_before_hours,
  }
}

pub fn folder_to_dto(folder: &Folder) -> FolderDto {
  FolderDto {
    id: folder.id.clone(),
    name: folder.name.clone(),
    icon: folder_icon_to_dto(folder.icon),
    updated_at_ms: folder.updated_at_ms,
    order: folder.order,
  }
}

pub fn folder_from_dto(dto: FolderDto) -> Folder {
  Folder { id: dto.id, name: dto.name, icon: folder_icon_from_dto(dto.icon), updated_at_ms: dto.updated_at_ms, order: dto.order }
}

pub fn tag_to_dto(tag: &Tag) -> TagDto {
  TagDto { id: tag.id.clone(), name: tag.name.clone(), updated_at_ms: tag.updated_at_ms, order: tag.order }
}

pub fn tag_from_dto(dto: TagDto) -> Tag {
  Tag { id: dto.id, name: dto.name, updated_at_ms: dto.updated_at_ms, order: dto.order }
}

pub fn build_change(
  device_id: &str,
  entity: EntityKind,
  entity_id: String,
  op: ChangeOp,
  payload: Option<ChangePayload>,
  client_updated_at_ms: i64,
) -> QueuedChange {
  QueuedChange {
    change_id: uuid::Uuid::new_v4().to_string(),
    device_id: device_id.to_string(),
    entity,
    entity_id,
    op,
    payload,
    client_updated_at_ms,
    enqueued_at_ms: now_ms(),
  }
}

pub fn diff_notes(previous: &[Note], current: &[Note], device_id: &str) -> Vec<QueuedChange> {
  let previous_by_id: HashMap<&str, &Note> = previous.iter().map(|note| (note.id.as_str(), note)).collect();
  let current_ids: HashSet<&str> = current.iter().map(|note| note.id.as_str()).collect();

  let mut changes: Vec<QueuedChange> = current
    .iter()
    .filter_map(|note| {
      match previous_by_id.get(note.id.as_str()) {
        None => Some(ChangeOp::Create),
        Some(previous_note) if *previous_note != note => Some(ChangeOp::Update),
        _ => None,
      }
      .map(|op| build_change(device_id, EntityKind::Note, note.id.clone(), op, Some(ChangePayload::Note(note_to_dto(note))), note.updated_at_ms))
    })
    .collect();

  changes.extend(
    previous
      .iter()
      .filter(|note| !current_ids.contains(note.id.as_str()))
      .map(|note| build_change(device_id, EntityKind::Note, note.id.clone(), ChangeOp::Delete, None, now_ms())),
  );

  changes
}

pub fn diff_folders(previous: &[Folder], current: &[Folder], device_id: &str) -> Vec<QueuedChange> {
  let previous_by_id: HashMap<&str, &Folder> = previous.iter().map(|folder| (folder.id.as_str(), folder)).collect();
  let current_ids: HashSet<&str> = current.iter().map(|folder| folder.id.as_str()).collect();

  let mut changes: Vec<QueuedChange> = current
    .iter()
    .filter_map(|folder| {
      match previous_by_id.get(folder.id.as_str()) {
        None => Some(ChangeOp::Create),
        Some(previous_folder) if *previous_folder != folder => Some(ChangeOp::Update),
        _ => None,
      }
      .map(|op| {
        build_change(device_id, EntityKind::Folder, folder.id.clone(), op, Some(ChangePayload::Folder(folder_to_dto(folder))), folder.updated_at_ms)
      })
    })
    .collect();

  changes.extend(
    previous
      .iter()
      .filter(|folder| !current_ids.contains(folder.id.as_str()))
      .map(|folder| build_change(device_id, EntityKind::Folder, folder.id.clone(), ChangeOp::Delete, None, now_ms())),
  );

  changes
}

pub fn diff_tags(previous: &[Tag], current: &[Tag], device_id: &str) -> Vec<QueuedChange> {
  let previous_by_id: HashMap<&str, &Tag> = previous.iter().map(|tag| (tag.id.as_str(), tag)).collect();
  let current_ids: HashSet<&str> = current.iter().map(|tag| tag.id.as_str()).collect();

  let mut changes: Vec<QueuedChange> = current
    .iter()
    .filter_map(|tag| {
      match previous_by_id.get(tag.id.as_str()) {
        None => Some(ChangeOp::Create),
        Some(previous_tag) if *previous_tag != tag => Some(ChangeOp::Update),
        _ => None,
      }
      .map(|op| build_change(device_id, EntityKind::Tag, tag.id.clone(), op, Some(ChangePayload::Tag(tag_to_dto(tag))), tag.updated_at_ms))
    })
    .collect();

  changes.extend(
    previous
      .iter()
      .filter(|tag| !current_ids.contains(tag.id.as_str()))
      .map(|tag| build_change(device_id, EntityKind::Tag, tag.id.clone(), ChangeOp::Delete, None, now_ms())),
  );

  changes
}

pub fn compute_next_id(notes: &[Note], folders: &[Folder], tags: &[Tag]) -> u32 {
  let extract = |id: &str| -> u32 { id.rsplit('-').next().and_then(|part| part.parse::<u32>().ok()).unwrap_or(0) };

  let max_note = notes.iter().map(|note| extract(&note.id)).max().unwrap_or(0);
  let max_folder = folders.iter().map(|folder| extract(&folder.id)).max().unwrap_or(0);
  let max_tag = tags.iter().map(|tag| extract(&tag.id)).max().unwrap_or(0);

  max_note.max(max_folder).max(max_tag) + 1
}

pub fn apply_server_changes(notes: &mut Vec<Note>, folders: &mut Vec<Folder>, tags: &mut Vec<Tag>, changes: Vec<ServerChange>) {
  for change in changes {
    match change.entity {
      EntityKind::Note => match change.op {
        ChangeOp::Delete => notes.retain(|note| note.id != change.entity_id),
        _ => {
          if let Some(ChangePayload::Note(dto)) = change.payload {
            let note = note_from_dto(dto);
            match notes.iter_mut().find(|existing| existing.id == note.id) {
              Some(existing) => *existing = note,
              None => notes.push(note),
            }
          }
        }
      },
      EntityKind::Folder => match change.op {
        ChangeOp::Delete => folders.retain(|folder| folder.id != change.entity_id),
        _ => {
          if let Some(ChangePayload::Folder(dto)) = change.payload {
            let folder = folder_from_dto(dto);
            match folders.iter_mut().find(|existing| existing.id == folder.id) {
              Some(existing) => *existing = folder,
              None => folders.push(folder),
            }
          }
        }
      },
      EntityKind::Tag => match change.op {
        ChangeOp::Delete => tags.retain(|tag| tag.id != change.entity_id),
        _ => {
          if let Some(ChangePayload::Tag(dto)) = change.payload {
            let tag = tag_from_dto(dto);
            match tags.iter_mut().find(|existing| existing.id == tag.id) {
              Some(existing) => *existing = tag,
              None => tags.push(tag),
            }
          }
        }
      },
    }
  }
}

pub fn merge_server_changes(snapshot: &mut PersistedState, changes: Vec<ServerChange>) {
  apply_server_changes(&mut snapshot.notes, &mut snapshot.folders, &mut snapshot.tags, changes);
  let next_id = compute_next_id(&snapshot.notes, &snapshot.folders, &snapshot.tags);
  snapshot.next_id = snapshot.next_id.max(next_id);
}
