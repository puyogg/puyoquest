-- Add migration script here

CREATE TABLE IF NOT EXISTS bot.leaderboard_channel (
    server_id TEXT NOT NULL,
    game_type TEXT NOT NULL,
    channel_id TEXT NOT NULL,
    PRIMARY KEY (server_id, game_type),
    CONSTRAINT fk_server_settings FOREIGN KEY (server_id) REFERENCES bot.server_settings(server_id) ON DELETE CASCADE
);
