SET timezone = 'UTC';
CREATE TABLE IF NOT EXISTS characters (
  char_id TEXT PRIMARY KEY,
  name TEXT,
  link_name TEXT,
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
  rarity_modifier TEXT,
  name TEXT,
  name_normalized TEXT,
  jp_name TEXT,
  jp_name_normalized TEXT,
  link_name TEXT,
  link_name_normalized TEXT,
  card_type TEXT,
  updated_at TIMESTAMPTZ,
  main_color TEXT,
  side_color TEXT,
  CONSTRAINT fk_characters FOREIGN KEY(char_id) REFERENCES characters(char_id) ON DELETE CASCADE
);
CREATE INDEX ON cards (char_id);
CREATE INDEX ON cards (name_normalized);
CREATE INDEX ON cards (jp_name_normalized);
CREATE INDEX ON cards (link_name_normalized);

CREATE TABLE IF NOT EXISTS aliases (
  alias TEXT PRIMARY KEY,
  char_id TEXT,
  internal BOOLEAN,
  card_type TEXT,
  updated_at TIMESTAMPTZ,
  CONSTRAINT fk_characters FOREIGN KEY(char_id) REFERENCES characters(char_id) ON DELETE CASCADE
);
CREATE INDEX ON aliases (char_id);

CREATE TABLE IF NOT EXISTS image_cache (
  external_url TEXT PRIMARY KEY,
  filepath TEXT,
  updated_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS enabled_roles (
  role_id TEXT PRIMARY KEY,
  guild_id TEXT,
  updated_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS cron_last_updated (
  task TEXT PRIMARY KEY,
  updated_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS ntc_leaderboard (
  user_id TEXT,
  server_id TEXT,
  correct INTEGER,
  PRIMARY KEY(user_id, server_id)
);

CREATE TABLE IF NOT EXISTS server_settings (
  server_id TEXT PRIMARY KEY,
  leaderboard_channel TEXT
);

INSERT INTO cron_last_updated (task, updated_at)
VALUES ('wiki_recent_changes', '2022-01-01 00:00:00.00+00')
ON CONFLICT (task) DO NOTHING;

-- 2/20/2023
ALTER TABLE cards ADD COLUMN IF NOT EXISTS main_color TEXT;
ALTER TABLE cards ADD COLUMN IF NOT EXISTS side_color TEXT;

-- 11/26/2023
CREATE TABLE IF NOT EXISTS kaga (
  kaga_id TEXT PRIMARY KEY,
  url TEXT
);
