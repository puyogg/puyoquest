DATABASE_URL := env('DATABASE_URL', 'postgres://postgres:password@localhost:35432/ppq_api_db')

@short-sha:
  git rev-parse --short HEAD

# Install this repo's recommended dev tools
@install-tools:
  cargo install sqlx-cli
  cargo install cargo-debugger

# Start the Postgres and Redis containers in the background
db-up:
  docker compose up -d api-db api-cache bot-db

# Migrate the api database
[working-directory: 'services/api']
migrate-api-db $DATABASE_URL=DATABASE_URL:
  sqlx migrate run

# Start the api
[working-directory: 'services/api']
api:
  cargo run --bin api

# Start the PPQ Discord Bot
[working-directory: 'services/bot']
bot:
  cargo run

# Start the bot's api
[working-directory: 'services/bot_api']
bot-api:
  cargo run --bin bot_api
