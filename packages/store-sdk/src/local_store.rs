use std::path::Path;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, Row, SqlitePool};
use sync_dto::{ChangeOp, ChangePayload, EntityKind, FolderDto, FolderIconDto, NoteDto, QueuedChange, TagDto};

pub struct LocalStore {
  pool: SqlitePool,
}

pub enum ApplyOutcome {
  Applied,
  Skipped,
  AlreadyApplied,
}

#[derive(Debug, Clone, Default)]
pub struct LocalSnapshot {
  pub notes: Vec<NoteDto>,
  pub folders: Vec<FolderDto>,
  pub tags: Vec<TagDto>,
}

impl LocalStore {
  pub async fn connect(db_path: &Path) -> Self {
    if let Some(parent) = db_path.parent() {
      std::fs::create_dir_all(parent).expect("failed to create local store directory");
    }

    let options = SqliteConnectOptions::new().filename(db_path).create_if_missing(true).disable_statement_logging();
    let pool = SqlitePoolOptions::new().connect_with(options).await.expect("failed to open local sqlite store");

    sqlx::migrate!("./migrations").run(&pool).await.expect("failed to run local store migrations");

    Self { pool }
  }

  pub async fn load_snapshot(&self) -> LocalSnapshot {
    let note_rows = sqlx::query(
      "SELECT id, title, content, folder_id, tag_ids, pinned, starred, updated_at_ms, sort_order FROM notes ORDER BY sort_order DESC",
    )
      .fetch_all(&self.pool)
      .await
      .expect("failed to load notes");

    let notes = note_rows
      .into_iter()
      .map(|row| NoteDto {
        id: row.get("id"),
        title: row.get("title"),
        content: row.get("content"),
        folder_id: row.get("folder_id"),
        tag_ids: serde_json::from_str(row.get::<String, _>("tag_ids").as_str()).unwrap_or_default(),
        pinned: row.get("pinned"),
        starred: row.get("starred"),
        updated_at_ms: row.get("updated_at_ms"),
        order: row.get("sort_order"),
      })
      .collect();

    let folder_rows = sqlx::query("SELECT id, name, icon, updated_at_ms, sort_order FROM folders ORDER BY sort_order ASC")
      .fetch_all(&self.pool)
      .await
      .expect("failed to load folders");

    let folders = folder_rows
      .into_iter()
      .map(|row| FolderDto {
        id: row.get("id"),
        name: row.get("name"),
        icon: serde_json::from_str(row.get::<String, _>("icon").as_str()).unwrap_or(FolderIconDto::Inbox),
        updated_at_ms: row.get("updated_at_ms"),
        order: row.get("sort_order"),
      })
      .collect();

    let tag_rows = sqlx::query("SELECT id, name, updated_at_ms, sort_order FROM tags ORDER BY sort_order ASC")
      .fetch_all(&self.pool)
      .await
      .expect("failed to load tags");

    let tags = tag_rows
      .into_iter()
      .map(|row| TagDto {
        id: row.get("id"),
        name: row.get("name"),
        updated_at_ms: row.get("updated_at_ms"),
        order: row.get("sort_order"),
      })
      .collect();

    LocalSnapshot { notes, folders, tags }
  }

  pub async fn apply(&self, change: &QueuedChange) -> ApplyOutcome {
    let already_applied = sqlx::query("SELECT 1 FROM applied_changes WHERE change_id = ?")
      .bind(&change.change_id)
      .fetch_optional(&self.pool)
      .await
      .expect("failed to query applied_changes")
      .is_some();

    if already_applied {
      return ApplyOutcome::AlreadyApplied;
    }

    let outcome = if change.op == ChangeOp::Delete {
      self.delete_entity(change.entity, &change.entity_id).await;
      ApplyOutcome::Applied
    } else {
      let existing_ms = self.existing_updated_at_ms(change.entity, &change.entity_id).await;
      if sync_dto::is_newer_or_equal(existing_ms, change.client_updated_at_ms) {
        self.upsert_payload(change.payload.as_ref()).await;
        ApplyOutcome::Applied
      } else {
        ApplyOutcome::Skipped
      }
    };

    sqlx::query("INSERT INTO applied_changes (change_id) VALUES (?)")
      .bind(&change.change_id)
      .execute(&self.pool)
      .await
      .expect("failed to record applied change");

    outcome
  }

  async fn existing_updated_at_ms(&self, entity: EntityKind, entity_id: &str) -> Option<i64> {
    let query = match entity {
      EntityKind::Note => "SELECT updated_at_ms FROM notes WHERE id = ?",
      EntityKind::Folder => "SELECT updated_at_ms FROM folders WHERE id = ?",
      EntityKind::Tag => "SELECT updated_at_ms FROM tags WHERE id = ?",
    };

    sqlx::query(query)
      .bind(entity_id)
      .fetch_optional(&self.pool)
      .await
      .expect("failed to query existing_updated_at_ms")
      .map(|row| row.get("updated_at_ms"))
  }

  async fn delete_entity(&self, entity: EntityKind, entity_id: &str) {
    let query = match entity {
      EntityKind::Note => "DELETE FROM notes WHERE id = ?",
      EntityKind::Folder => "DELETE FROM folders WHERE id = ?",
      EntityKind::Tag => "DELETE FROM tags WHERE id = ?",
    };

    sqlx::query(query).bind(entity_id).execute(&self.pool).await.expect("failed to delete entity");
  }

  async fn upsert_payload(&self, payload: Option<&ChangePayload>) {
    match payload {
      Some(ChangePayload::Note(note)) => self.upsert_note(note).await,
      Some(ChangePayload::Folder(folder)) => self.upsert_folder(folder).await,
      Some(ChangePayload::Tag(tag)) => self.upsert_tag(tag).await,
      None => {}
    }
  }

  async fn upsert_note(&self, note: &NoteDto) {
    let tag_ids = serde_json::to_string(&note.tag_ids).unwrap_or_else(|_| "[]".to_string());

    sqlx::query(
      "INSERT INTO notes (id, title, content, folder_id, tag_ids, pinned, starred, updated_at_ms, sort_order) \
       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) \
       ON CONFLICT(id) DO UPDATE SET title = excluded.title, content = excluded.content, folder_id = excluded.folder_id, \
       tag_ids = excluded.tag_ids, pinned = excluded.pinned, starred = excluded.starred, \
       updated_at_ms = excluded.updated_at_ms, sort_order = excluded.sort_order",
    )
    .bind(&note.id)
    .bind(&note.title)
    .bind(&note.content)
    .bind(&note.folder_id)
    .bind(tag_ids)
    .bind(note.pinned)
    .bind(note.starred)
    .bind(note.updated_at_ms)
    .bind(note.order)
    .execute(&self.pool)
    .await
    .expect("failed to upsert note");
  }

  async fn upsert_folder(&self, folder: &FolderDto) {
    let icon = serde_json::to_string(&folder.icon).unwrap_or_else(|_| "\"Inbox\"".to_string());

    sqlx::query(
      "INSERT INTO folders (id, name, icon, updated_at_ms, sort_order) VALUES (?, ?, ?, ?, ?) \
       ON CONFLICT(id) DO UPDATE SET name = excluded.name, icon = excluded.icon, updated_at_ms = excluded.updated_at_ms, sort_order = excluded.sort_order",
    )
    .bind(&folder.id)
    .bind(&folder.name)
    .bind(icon)
    .bind(folder.updated_at_ms)
    .bind(folder.order)
    .execute(&self.pool)
    .await
    .expect("failed to upsert folder");
  }

  async fn upsert_tag(&self, tag: &TagDto) {
    sqlx::query(
      "INSERT INTO tags (id, name, updated_at_ms, sort_order) VALUES (?, ?, ?, ?) \
       ON CONFLICT(id) DO UPDATE SET name = excluded.name, updated_at_ms = excluded.updated_at_ms, sort_order = excluded.sort_order",
    )
    .bind(&tag.id)
    .bind(&tag.name)
    .bind(tag.updated_at_ms)
    .bind(tag.order)
    .execute(&self.pool)
    .await
    .expect("failed to upsert tag");
  }

  pub async fn enqueue_outbound(&self, change: &QueuedChange) {
    let change_json = serde_json::to_string(change).expect("failed to serialize queued change");

    sqlx::query("INSERT INTO outbound_queue (change_json, enqueued_at_ms) VALUES (?, ?)")
      .bind(change_json)
      .bind(change.enqueued_at_ms)
      .execute(&self.pool)
      .await
      .expect("failed to enqueue outbound change");
  }

  pub async fn peek_front_outbound(&self) -> Option<(i64, QueuedChange)> {
    let row = sqlx::query("SELECT id, change_json FROM outbound_queue ORDER BY id ASC LIMIT 1")
      .fetch_optional(&self.pool)
      .await
      .expect("failed to peek outbound queue")?;

    let id: i64 = row.get("id");
    let change_json: String = row.get("change_json");
    let change = serde_json::from_str(&change_json).expect("corrupt queued change json");

    Some((id, change))
  }

  pub async fn dequeue_front_outbound(&self, id: i64) {
    sqlx::query("DELETE FROM outbound_queue WHERE id = ?")
      .bind(id)
      .execute(&self.pool)
      .await
      .expect("failed to dequeue outbound change");
  }

  pub async fn cursor(&self) -> i64 {
    sqlx::query("SELECT cursor FROM sync_cursor WHERE id = 0")
      .fetch_optional(&self.pool)
      .await
      .expect("failed to query sync cursor")
      .map(|row| row.get("cursor"))
      .unwrap_or(0)
  }

  pub async fn set_cursor(&self, cursor: i64) {
    sqlx::query("INSERT INTO sync_cursor (id, cursor) VALUES (0, ?) ON CONFLICT(id) DO UPDATE SET cursor = excluded.cursor")
      .bind(cursor)
      .execute(&self.pool)
      .await
      .expect("failed to set sync cursor");
  }

  pub async fn device_id(&self) -> String {
    let candidate = uuid::Uuid::new_v4().to_string();

    let row = sqlx::query(
      "INSERT INTO device_identity (id, device_id) VALUES (0, ?) \
       ON CONFLICT(id) DO UPDATE SET device_id = device_identity.device_id \
       RETURNING device_id",
    )
    .bind(&candidate)
    .fetch_one(&self.pool)
    .await
    .expect("failed to get or create device_id");

    row.get("device_id")
  }
}
