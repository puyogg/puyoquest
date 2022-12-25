# puyoquest

## Development

Install monorepo dependencies using Docker.

```bash
docker compose up
```

If you need to install new dependencies, do that from within the `base` container. Example:

```bash
docker compose run --rm base bash
cd packages/facade
pnpm add -D eslint
```

## Production

```
docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

### Initialize Databases

```bash
docker compose exec discordbot-db psql -U postgres -d discordbot-db < ./packages/database/sql/init.sql
```
