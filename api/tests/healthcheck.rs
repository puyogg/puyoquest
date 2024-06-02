use poem::{http::StatusCode, test::TestClient};

use api::init_api;

#[tokio::test]
async fn healthcheck_test() {
    let api = init_api();
    let client = TestClient::new(api);

    let response = client.get("/healthcheck").send().await;
    response.assert_status(StatusCode::OK);
    response.assert_text("OK!").await;
}
