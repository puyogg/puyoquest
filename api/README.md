# api

## Development

### Execute migrations and seeds

Install the [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli). Then run:

```sh
docker compose up -d api-db
cd api
export DATABASE_URL=postgres://postgres:password@localhost:35432/ppq_api_db
sqlx migrate run
```

### Start the API

```sh
cargo run -p api
```

### Testing

First run this command to initialize an "admin_db". This db lets you clone the primary db, which enables integration tests to run in parallel (Rust's default).

```sh
cargo run -p api --bin test_setup
```

Run integration tests:

```sh
cargo watch -x "test -p api --test '*'"
```
