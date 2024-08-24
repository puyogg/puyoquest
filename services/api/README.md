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
