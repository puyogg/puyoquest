const DB_PORT: &'static str = "35433";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = format!("postgres://postgres:password@localhost:{DB_PORT}/ppq_bot_db");
    let pool = sqlx::PgPool::connect(&connection_string).await?;
    let _ = sqlx::query("CREATE DATABASE admin_db")
        .execute(&pool)
        .await?;

    Ok(())
}
