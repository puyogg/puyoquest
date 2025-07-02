use bot_api::routes::kaga::KagaData;
use poem::http::StatusCode;

use crate::common::{IntTestResult, create_test_client};

#[tokio::test]
async fn puts_kaga_image() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let expected_kaga = KagaData {
        kaga_id: "1".to_string(),
        url: "http://example.com".to_string(),
    };

    let response = client.put("/kaga").body_json(&expected_kaga).send().await;

    response.assert_status_is_ok();
    response.assert_json(&expected_kaga).await;

    Ok(())
}

#[tokio::test]
async fn gets_kaga_image() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let expected_kaga = KagaData {
        kaga_id: "1".to_string(),
        url: "http://example.com".to_string(),
    };

    client.put("/kaga").body_json(&expected_kaga).send().await;

    let fetched_kaga = client
        .get("/kaga")
        .query("id", &expected_kaga.kaga_id)
        .send()
        .await;

    fetched_kaga.assert_status_is_ok();
    fetched_kaga.assert_json(&expected_kaga).await;

    Ok(())
}

#[tokio::test]
async fn kaga_not_found() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let response = client
        .get("/kaga")
        .query("id", &"asdf")
        .send()
        .await;

    response.assert_status(StatusCode::NOT_FOUND);
    response.assert_text("kaga_id asdf does not exist").await;

    Ok(())
}

#[tokio::test]
async fn delete_kaga_image() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let expected_kaga = KagaData {
        kaga_id: "1".to_string(),
        url: "http://example.com".to_string(),
    };

    client.put("/kaga").body_json(&expected_kaga).send().await;

    let fetched_kaga = client
        .get("/kaga")
        .query("id", &expected_kaga.kaga_id)
        .send()
        .await;
    fetched_kaga.assert_status_is_ok();

    client
        .delete("/kaga")
        .query("id", &expected_kaga.kaga_id)
        .send()
        .await;

    let fetched_kaga = client
        .get("/kaga")
        .query("id", &expected_kaga.kaga_id)
        .send()
        .await;
    fetched_kaga.assert_status(StatusCode::NOT_FOUND);

    Ok(())
}
