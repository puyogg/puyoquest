use poem::{http::StatusCode, test::TestClient};
use api::init_api;
use api::db::create_pool;

#[tokio::test]
async fn healthcheck_test() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_pool().await?;
    let api = init_api(pool);
    let client = TestClient::new(api);

    let response = client.get("/healthcheck").send().await;
    response.assert_status(StatusCode::OK);
    response.assert_text("OK!").await;

    Ok(())
}
