ALTER TABLE notes ADD COLUMN date_ms INTEGER NOT NULL DEFAULT 0;
ALTER TABLE notes ADD COLUMN remind_before_hours INTEGER;
UPDATE notes SET date_ms = updated_at_ms WHERE date_ms = 0;
