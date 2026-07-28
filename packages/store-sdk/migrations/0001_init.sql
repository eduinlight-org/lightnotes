CREATE TABLE notes (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  folder_id TEXT,
  tag_ids TEXT NOT NULL,
  pinned INTEGER NOT NULL,
  starred INTEGER NOT NULL,
  updated_at_ms INTEGER NOT NULL,
  sort_order INTEGER NOT NULL
);

CREATE TABLE folders (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  icon TEXT NOT NULL,
  updated_at_ms INTEGER NOT NULL
);

CREATE TABLE tags (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  updated_at_ms INTEGER NOT NULL
);

CREATE TABLE outbound_queue (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  change_json TEXT NOT NULL,
  enqueued_at_ms INTEGER NOT NULL
);

CREATE TABLE applied_changes (
  change_id TEXT PRIMARY KEY
);

CREATE TABLE sync_cursor (
  id INTEGER PRIMARY KEY CHECK (id = 0),
  cursor INTEGER NOT NULL
);

CREATE TABLE device_identity (
  id INTEGER PRIMARY KEY CHECK (id = 0),
  device_id TEXT NOT NULL
);
