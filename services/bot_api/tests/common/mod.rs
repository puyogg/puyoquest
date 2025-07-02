use std::sync::Arc;

use bot_api::{BotApi, init_api};
use poem::test::TestClient;
use uuid::Uuid;

const DB_PORT: &'static str = "35433";

pub type IntTestResult<T> = Result<T, Box<dyn std::error::Error>>;

pub async fn create_test_pool(db_name: &str) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let connection_string = format!("postgres://postgres:password@localhost:{DB_PORT}/{db_name}");
    sqlx::PgPool::connect(&connection_string).await
}

pub async fn request_test_db() -> IntTestResult<String> {
    let admin_pool = create_test_pool("admin_db").await?;

    let db_uuid = Uuid::new_v4().to_string();
    let db_name = format!("test_{db_uuid}").replace("-", "");

    let drop_query = format!("DROP DATABASE IF EXISTS {db_name}");
    sqlx::query(&drop_query).execute(&admin_pool).await?;

    let create_query = format!("CREATE DATABASE {db_name} TEMPLATE ppq_bot_db");
    sqlx::query(&create_query).execute(&admin_pool).await?;

    // Make sure the cloned db is clean.
    let cloned_db_pool = create_test_pool(&db_name).await?;
    sqlx::query("TRUNCATE TABLE bot.leaderboard CASCADE")
        .execute(&cloned_db_pool)
        .await?;
    sqlx::query("TRUNCATE TABLE bot.server_settings CASCADE")
        .execute(&cloned_db_pool)
        .await?;
    sqlx::query("TRUNCATE TABLE bot.leaderboard_channel CASCADE")
        .execute(&cloned_db_pool)
        .await?;
    sqlx::query("TRUNCATE TABLE bot.kaga CASCADE")
        .execute(&cloned_db_pool)
        .await?;

    admin_pool.close().await;
    cloned_db_pool.close().await;

    Ok(db_name)
}

pub type TestDbName = String;

pub async fn create_test_client() -> IntTestResult<TestClient<BotApi>> {
    let test_db_name = request_test_db().await?;
    let pool = create_test_pool(&test_db_name).await?;

    let sdk_config = aws_config::from_env().load().await;
    let aws_client = bot_api::aws::AwsClient::new(sdk_config);
    let aws_client = Arc::new(aws_client);

    let api = init_api(aws_client, pool);
    let client = TestClient::new(api);

    Ok(client)
}
