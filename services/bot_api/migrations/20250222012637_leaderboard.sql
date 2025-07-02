-- Add migration script here

CREATE TABLE IF NOT EXISTS bot.leaderboard (
  game_type TEXT NOT NULL,
  user_id TEXT NOT NULL,
  server_id TEXT NOT NULL,
  correct INTEGER NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY(user_id, server_id, game_type)
);

CREATE TRIGGER bot_leaderboard_updated_at
BEFORE UPDATE ON bot.leaderboard
FOR EACH ROW EXECUTE PROCEDURE on_update_timestamp();

CREATE TRIGGER bot_leaderboard_inserted_at
BEFORE INSERT ON bot.leaderboard
FOR EACH ROW EXECUTE PROCEDURE on_update_timestamp();
