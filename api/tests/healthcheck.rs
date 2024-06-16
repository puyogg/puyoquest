use api::db::create_pool;
use api::init_api;
use poem::{http::StatusCode, test::TestClient};
use wiki::wiki_client::WikiClient;

#[tokio::test]
async fn healthcheck_test() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_pool().await?;
    let wiki_client = WikiClient::new("N/A", "N/A");
    let api = init_api(pool, wiki_client);
    let client = TestClient::new(api);

    let response = client.get("/healthcheck").send().await;
    response.assert_status(StatusCode::OK);
    response.assert_text("OK!").await;

    Ok(())
}
