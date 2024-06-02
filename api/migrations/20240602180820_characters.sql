BEGIN;

CREATE TABLE IF NOT EXISTS character (
  char_id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  jp_name TEXT NOT NULL,
  link_name TEXT NOT NULL,
  main_color TEXT,
  side_color TEXT,
  type1 TEXT,
  type2 TEXT,
  voice_trans TEXT,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX ON character (name);
CREATE INDEX ON character (jp_name);

CREATE TRIGGER character_updated_at
BEFORE UPDATE ON character
FOR EACH ROW EXECUTE PROCEDURE on_update_timestamp();

COMMIT;