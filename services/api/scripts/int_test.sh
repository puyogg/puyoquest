set -e

DATABASE_URL=postgres://postgres:password@localhost:35432/ppq_api_db sqlx migrate run
cargo run -p api --bin test_setup
cargo test -p api --test '*' -- --nocapture
