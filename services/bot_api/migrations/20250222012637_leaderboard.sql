-- Add migration script here

CREATE TABLE IF NOT EXISTS bot.leaderboard (
  game_type TEXT NOT NULL,
  user_id TEXT NOT NULL,
  server_id TEXT NOT NULL,
  correct INTEGER NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY(user_id, server_id, game_type)
);
