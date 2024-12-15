# api

## Data Model

```mermaid
erDiagram
  character ||--o{ card : "has many"
  character ||--o{ alias : "also called by"
  character {
    string char_id PK
    string name
    string jp_name
    string link_name
    string main_color
    date updated_at
    _ _
  }

  card ||--o{ card_category: "in many"
  card ||--o{ image_cache: ""
  card {
    string card_id PK
    string char_id FK
    string rarity
    string rarity_modifier
    string name_normalized
    json wiki_template
    date updated_at
    _ _
  }

  alias {
    string alias PK
    string char_id FK
    _ _
  }

  card_category {
    string card_id FK
    string char_id FK
    string category "UNIQUE(card_id, category)"
  }

  image_cache {
    string filepath PK
    string card_id FK
    string bucket_url
    enum image_type "(portrait, card)"
    enum image_subtype "(left, right, fp)"
    date updated_at
  }
```

## Development

### Execute migrations and seeds

Install the [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli). Then run:

```sh
docker compose up -d api-db
cd services/api
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

#### Notes on writing tests

**Make sure you use poem's ToJSON trait when comparing dates in tests**. `serde_json::to_value` and poem's `response.assert_json`, calls the implemented serialization, which for chrono::DateTime, [adds the trailing z](https://github.com/chronotope/chrono/blob/8b863490d88ba098038392c8aa930012ffd0c439/src/datetime/serde.rs#L41-L47). Poem's response handlers, on the other hand, chooses to serialize chrono::DateTime by [calling to_rfc3339() directly](https://github.com/poem-web/poem/blob/ff6071f7e4cfd677946fa9ec0f4a47224fb596cb/poem-openapi/src/types/external/chrono.rs#L69-L73), which adds a trailing +00:00 instead. `to_json()` needs to be called in tests in advance to keep the format consistent.

```rust
use poem_openapi::types::ToJSON;

let response = client.get("/").send().await;
response.assert_json(some_obj.to_json().unwrap).await;
```

### Building for Release

I deploy the API to a t4g ec2 instance. They run on arm64 gnu linux

Install the cross tool.

```sh
cross build -p api --bin api --release --target aarch64-unknown-linux-gnu
aws s3 cp ./target/aarch64-unknown-linux-gnu/release/api s3://api-bin/api-linux-gnu-latest
```

### Legacy DB Migration

```sh
# Remote server
NOW=$(date +%s)
docker compose exec discordbot-db pg_dump -U postgres discordbot-db > backup_${NOW}.sql

# Locally
docker compose up -d legacy-db
docker compose exec -T legacy-db psql -U postgres -d discordbot-db < backup_1234567890.sql
docker compose exec -T legacy-db psql -U postgres -d discordbot-db < ./services/api/legacy-migration/legacy-db-migration.sql
docker compose exec legacy-db pg_dump -U postgres discordbot-db > migrated.sql
docker compose exec -T api-db psql -U postgres -d ppq_api_db < migrated.sql
```

If you mess up:

```sh
docker compose down -v
```
