-- Add migration script here
BEGIN;

CREATE TABLE IF NOT EXISTS alias (
  alias TEXT PRIMARY KEY,
  char_id TEXT NOT NULL,
  internal BOOLEAN NOT NULL,
  card_type card_type NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_characters FOREIGN KEY (char_id) REFERENCES character(char_id) ON DELETE CASCADE
);

CREATE TRIGGER alias_updated_at
BEFORE UPDATE ON alias
FOR EACH ROW EXECUTE PROCEDURE on_update_timestamp();

CREATE TRIGGER alias_inserted_at
BEFORE INSERT ON alias
FOR EACH ROW EXECUTE PROCEDURE on_update_timestamp();

COMMIT;