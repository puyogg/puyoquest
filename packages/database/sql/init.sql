SET timezone = 'UTC';

CREATE TABLE IF NOT EXISTS characters (
  char_id TEXT PRIMARY KEY,
  name TEXT,
  jp_name TEXT,
  main_color TEXT,
  side_color TEXT,
  type1 TEXT,
  type2 TEXT,
  voice_trans TEXT,
  updated_at TIMESTAMPTZ
);
CREATE INDEX ON characters (name);
CREATE INDEX ON characters (jp_name);

CREATE TABLE IF NOT EXISTS cards (
  card_id TEXT PRIMARY KEY,
  char_id TEXT,
  rarity TEXT,
  name TEXT,
  name_normalized TEXT,
  jp_name TEXT,
  link_name TEXT,
  card_type TEXT,
  updated_at TIMESTAMPTZ,
  CONSTRAINT fk_characters
    FOREIGN KEY(char_id)
      REFERENCES characters(char_id)
      ON DELETE CASCADE
);
CREATE INDEX ON cards (char_id);
CREATE INDEX ON cards (name_normalized);
CREATE INDEX ON cards (jp_name);
CREATE INDEX ON cards (link_name);

CREATE TABLE IF NOT EXISTS aliases (
  alias TEXT PRIMARY KEY,
  char_id TEXT,
  updated_at TIMESTAMPTZ,
  CONSTRAINT fk_characters
    FOREIGN KEY(char_id)
      REFERENCES characters(char_id)
      ON DELETE CASCADE
);
CREATE INDEX ON aliases (char_id);

CREATE TABLE IF NOT EXISTS image_cache (
  external_url TEXT PRIMARY KEY,
  filepath TEXT,
  updated_at TIMESTAMPTZ
);
