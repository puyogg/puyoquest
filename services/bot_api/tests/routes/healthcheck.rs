use poem::http::StatusCode;

use crate::common::{IntTestResult, create_test_client};

#[tokio::test]
async fn healthcheck() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let response = client.get("/healthcheck").send().await;
    response.assert_status(StatusCode::OK);
    response.assert_text("OK!!").await;

    Ok(())
}
