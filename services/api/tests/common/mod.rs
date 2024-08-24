use api::{
    db::{create_pool_from_opts, PoolOpts},
    init_api,
};

use poem::test::TestClient;
use uuid::Uuid;
use wiki::wiki_client::WikiClient;
pub mod seed;

#[allow(dead_code)]
pub type IntTestResult = Result<(), Box<dyn std::error::Error>>;

pub async fn create_test_pool(db_name: &str) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    create_pool_from_opts(PoolOpts {
        username: "postgres",
        password: "password",
        host: "localhost",
        port: "35432",
        db: db_name,
    })
    .await
}

pub async fn request_test_db() -> Result<String, Box<dyn std::error::Error>> {
    let admin_pool = create_pool_from_opts(PoolOpts {
        username: "postgres",
        password: "password",
        host: "localhost",
        port: "35432",
        db: "admin_db",
    })
    .await
    .unwrap();

    let db_uuid = Uuid::new_v4().to_string();
    let db_name = format!("test_{db_uuid}").replace("-", "");

    let drop_query = format!("DROP DATABASE IF EXISTS {db_name}");
    sqlx::query(&drop_query).execute(&admin_pool).await?;

    let create_query = format!("CREATE DATABASE {db_name} TEMPLATE ppq_api_db");
    sqlx::query(&create_query).execute(&admin_pool).await?;

    // Make sure the cloned db is clean.
    let cloned_db_pool = create_test_pool(&db_name).await?;
    sqlx::query("TRUNCATE TABLE character CASCADE")
        .execute(&cloned_db_pool)
        .await?;
    sqlx::query("TRUNCATE TABLE card CASCADE")
        .execute(&cloned_db_pool)
        .await?;
    sqlx::query("TRUNCATE TABLE alias CASCADE")
        .execute(&cloned_db_pool)
        .await?;

    admin_pool.close().await;
    cloned_db_pool.close().await;

    Ok(db_name)
}

#[allow(dead_code)]
pub type TestDbName = String;

#[allow(dead_code)]
pub async fn create_test_client(
    pn_api_url: &str,
    pn_base_url: &str,
) -> Result<(TestClient<api::Api>, TestDbName), Box<dyn std::error::Error>> {
    let test_db_name = request_test_db().await?;
    let pool = create_test_pool(&test_db_name).await?;
    let wiki_client = WikiClient::new(pn_api_url, pn_base_url);
    let api = init_api(pool, wiki_client);
    let client = TestClient::new(api);

    Ok((client, test_db_name))
}
