use bot_api::routes::server_settings::ServerSettings;
use poem::http::StatusCode;

use crate::common::{IntTestResult, create_test_client};

#[tokio::test]
async fn upsert_server_settings() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let expected_settings = ServerSettings {
        server_id: "12345".to_string(),
    };

    let response = client
        .post("/server-settings")
        .body_json(&expected_settings)
        .send()
        .await;

    response.assert_status_is_ok();
    response.assert_json(&expected_settings).await;

    Ok(())
}

#[tokio::test]
async fn fetch_server_settings() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let expected_settings = ServerSettings {
        server_id: "12345".to_string(),
    };

    client
        .post("/server-settings")
        .body_json(&expected_settings)
        .send()
        .await;

    let response = client
        .get(format!("/server-settings/{}", expected_settings.server_id))
        .send()
        .await;

    response.assert_status_is_ok();
    response.assert_json(&expected_settings).await;

    Ok(())
}

#[tokio::test]
async fn delete_server_settings() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let expected_settings = ServerSettings {
        server_id: "12345".to_string(),
    };

    client
        .post("/server-settings")
        .body_json(&expected_settings)
        .send()
        .await;

    let fetch_response = client
        .get(format!("/server-settings/{}", expected_settings.server_id))
        .send()
        .await;
    fetch_response.assert_status_is_ok();

    client
        .delete(format!("/server-settings/{}", expected_settings.server_id))
        .send()
        .await;

    let fetch_response = client
        .get(format!("/server-settings/{}", expected_settings.server_id))
        .send()
        .await;
    fetch_response.assert_status(StatusCode::NOT_FOUND);

    Ok(())
}
