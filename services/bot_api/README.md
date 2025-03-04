# bot_api

```sh
docker compose up -d bot-db
cd services/bot_api
export DATABASE_URL=postgres://postgres:password@localhost:35433/ppq_bot_db
sqlx migrate run
```
