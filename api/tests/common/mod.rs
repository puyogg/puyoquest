use api::db::{create_pool_from_opts, PoolOpts};

use uuid::Uuid;
pub mod seed;

pub async fn create_test_pool(db_name: &str) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    create_pool_from_opts(PoolOpts {
        username: "postgres",
        password: "password",
        host: "localhost",
        port: "35432",
        db: db_name,
    }).await
}

pub async fn request_test_db() -> Result<String, Box<dyn std::error::Error>> {
    let admin_pool = create_pool_from_opts(PoolOpts {
        username: "postgres",
        password: "password",
        host: "localhost",
        port: "35432",
        db: "admin_db",
    }).await.unwrap();

    let db_uuid = Uuid::new_v4().to_string();
    let db_name = format!("test_{db_uuid}").replace("-", "");

    let drop_query = format!("DROP DATABASE IF EXISTS {db_name}");
    let _ = sqlx::query(&drop_query).execute(&admin_pool).await;

    let create_query = format!("CREATE DATABASE {db_name} TEMPLATE ppq_api_db");
    let _ = sqlx::query(&create_query).execute(&admin_pool).await;

    // Make sure the cloned db is clean.
    let cloned_db_pool = create_test_pool(&db_name).await?;
    let _ = sqlx::query("TRUNCATE TABLE character CASCADE").execute(&cloned_db_pool).await;
    let _ = sqlx::query("TRUNCATE TABLE card CASCADE").execute(&cloned_db_pool).await;

    admin_pool.close().await;
    cloned_db_pool.close().await;

    Ok(db_name)
}
