-- Add migration script here

CREATE TABLE IF NOT EXISTS bot.server_settings (
    server_id TEXT NOT NULL,
    PRIMARY KEY(server_id)
);
