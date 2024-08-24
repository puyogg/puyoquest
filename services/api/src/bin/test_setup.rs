use api::db::create_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_pool().await?;
    let _ = sqlx::query("CREATE DATABASE admin_db")
        .execute(&pool)
        .await?;

    Ok(())
}
