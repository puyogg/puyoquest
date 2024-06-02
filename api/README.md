# api

## Development

### Quickstart

#### Execute migrations and seeds

Install the [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli). Then run:

```sh
docker compose up -d api-db
cd api
export DATABASE_URL=postgres://postgres:password@localhost:35432/ppq_api_db
sqlx migrate run
```

#### Start the API

```sh
cargo run -p api
```

### Tests

### Adding new tables

###
