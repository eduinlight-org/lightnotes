DROP TABLE notes;
DROP TABLE folders;
DROP TABLE tags;
DROP TABLE outbound_queue;
DROP TABLE applied_changes;
DROP TABLE sync_cursor;

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
