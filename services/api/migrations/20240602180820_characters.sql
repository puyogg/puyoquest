BEGIN;

CREATE TABLE IF NOT EXISTS character (
  char_id TEXT PRIMARY KEY,
  name TEXT,
  jp_name TEXT,
  link_name TEXT,
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

CREATE TRIGGER character_inserted_at
BEFORE INSERT ON character
FOR EACH ROW EXECUTE PROCEDURE on_update_timestamp();

COMMIT;