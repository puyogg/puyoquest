# bot

Discord bot client for the Puyo Quest API.

## Development

Make a copy of `.env.example` as `.env` and fill out the variables. You will need to get `CLIENT_ID` and `BOT_TOKEN` from the Discord Developer Dashboard.

Invite the bot to your primary server with a Discord bot invite link: `TODO`.

Start the database:

```sh
docker compose TODO
```

Then start the bot. NOTE: The `dotenvy` package looks for the `.env` file based on your CWD.

```sh
cd services/bot
cargo run
```

### Notes on upgrading packages

- poise and poise_macros should be upgraded together
