ALTER TABLE notes RENAME TO notes_legacy;
ALTER TABLE folders RENAME TO folders_legacy;
ALTER TABLE tags RENAME TO tags_legacy;
ALTER TABLE outbound_queue RENAME TO outbound_queue_legacy;
ALTER TABLE applied_changes RENAME TO applied_changes_legacy;
ALTER TABLE sync_cursor RENAME TO sync_cursor_legacy;

CREATE TABLE notes (
  user_id TEXT NOT NULL,
  id TEXT NOT NULL,
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  folder_id TEXT,
  tag_ids TEXT NOT NULL,
  pinned INTEGER NOT NULL,
  starred INTEGER NOT NULL,
  updated_at_ms INTEGER NOT NULL,
  sort_order INTEGER NOT NULL,
  date_ms INTEGER NOT NULL DEFAULT 0,
  remind_before_hours INTEGER,
  PRIMARY KEY (user_id, id)
);

CREATE TABLE folders (
  user_id TEXT NOT NULL,
  id TEXT NOT NULL,
  name TEXT NOT NULL,
  icon TEXT NOT NULL,
  updated_at_ms INTEGER NOT NULL,
  sort_order INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (user_id, id)
);

CREATE TABLE tags (
  user_id TEXT NOT NULL,
  id TEXT NOT NULL,
  name TEXT NOT NULL,
  updated_at_ms INTEGER NOT NULL,
  sort_order INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (user_id, id)
);

CREATE TABLE outbound_queue (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id TEXT NOT NULL,
  change_json TEXT NOT NULL,
  enqueued_at_ms INTEGER NOT NULL
);

CREATE INDEX idx_outbound_queue_user ON outbound_queue (user_id, id);

CREATE TABLE applied_changes (
  user_id TEXT NOT NULL,
  change_id TEXT NOT NULL,
  PRIMARY KEY (user_id, change_id)
);

CREATE TABLE sync_cursor (
  user_id TEXT PRIMARY KEY,
  cursor INTEGER NOT NULL
);

INSERT INTO notes (user_id, id, title, content, folder_id, tag_ids, pinned, starred, updated_at_ms, sort_order, date_ms, remind_before_hours)
SELECT s.user_id, l.id, l.title, l.content, l.folder_id, l.tag_ids, l.pinned, l.starred, l.updated_at_ms, l.sort_order, l.date_ms, l.remind_before_hours
FROM notes_legacy l JOIN session s ON s.id = 0;

INSERT INTO folders (user_id, id, name, icon, updated_at_ms, sort_order)
SELECT s.user_id, l.id, l.name, l.icon, l.updated_at_ms, l.sort_order
FROM folders_legacy l JOIN session s ON s.id = 0;

INSERT INTO tags (user_id, id, name, updated_at_ms, sort_order)
SELECT s.user_id, l.id, l.name, l.updated_at_ms, l.sort_order
FROM tags_legacy l JOIN session s ON s.id = 0;

INSERT INTO outbound_queue (id, user_id, change_json, enqueued_at_ms)
SELECT l.id, s.user_id, l.change_json, l.enqueued_at_ms
FROM outbound_queue_legacy l JOIN session s ON s.id = 0;

INSERT INTO applied_changes (user_id, change_id)
SELECT s.user_id, l.change_id
FROM applied_changes_legacy l JOIN session s ON s.id = 0;

INSERT INTO sync_cursor (user_id, cursor)
SELECT s.user_id, l.cursor
FROM sync_cursor_legacy l JOIN session s ON s.id = 0
WHERE l.id = 0;

DROP TABLE notes_legacy;
DROP TABLE folders_legacy;
DROP TABLE tags_legacy;
DROP TABLE outbound_queue_legacy;
DROP TABLE applied_changes_legacy;
DROP TABLE sync_cursor_legacy;
