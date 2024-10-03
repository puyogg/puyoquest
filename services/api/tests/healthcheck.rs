mod common;

use poem::http::StatusCode;
use crate::common::create_test_client;

#[tokio::test]
async fn healthcheck_test() -> Result<(), Box<dyn std::error::Error>> {
    let (client, _) = create_test_client("N/A", "N/A").await?;

    let response = client.get("/healthcheck").send().await;
    response.assert_status(StatusCode::OK);
    response.assert_text("OK!").await;

    Ok(())
}
