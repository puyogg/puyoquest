-- Add migration script here

CREATE TABLE IF NOT EXISTS leaderboard (
  game_type TEXT NOT NULL,
  user_id TEXT NOT NULL,
  server_id TEXT NOT NULL,
  correct INTEGER NOT NULL,
  PRIMARY KEY(user_id, server_id, game_type)
);
