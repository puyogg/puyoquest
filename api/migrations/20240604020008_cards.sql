-- Add migration script here
BEGIN;

CREATE TYPE card_type AS ENUM ('character', 'material');

CREATE TABLE IF NOT EXISTS card (
  card_id TEXT PRIMARY KEY,
  char_id TEXT NOT NULL,
  rarity TEXT NOT NULL,
  rarity_modifier TEXT,
  name TEXT NOT NULL,
  name_normalized TEXT NOT NULL,
  jp_name TEXT,
  jp_name_normalized TEXT,
  link_name TEXT NOT NULL,
  link_name_normalized TEXT NOT NULL,
  card_type card_type NOT NULL,
  main_color TEXT NOT NULL,
  side_color TEXT,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  wiki_template JSONB,
  CONSTRAINT fk_characters FOREIGN KEY (char_id) REFERENCES character(char_id) ON DELETE CASCADE
);

CREATE INDEX on card (char_id);
CREATE INDEX on card (name_normalized);
CREATE INDEX on card (jp_name_normalized);
CREATE INDEX on card (link_name_normalized);

CREATE TRIGGER card_updated_at
BEFORE UPDATE ON card
FOR EACH ROW EXECUTE PROCEDURE on_update_timestamp();

CREATE TRIGGER card_inserted_at
BEFORE UPDATE ON card
FOR EACH ROW EXECUTE PROCEDURE on_update_timestamp();

COMMIT;
