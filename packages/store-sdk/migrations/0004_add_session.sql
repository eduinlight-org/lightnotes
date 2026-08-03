CREATE TABLE session (
  id INTEGER PRIMARY KEY CHECK (id = 0),
  user_id TEXT NOT NULL,
  session_json TEXT NOT NULL,
  updated_at_ms INTEGER NOT NULL
);
