use api::aliases::types::AliasCreate;
use api::characters::types::CharacterCreate;
use poem_openapi::types::ToJSON;
use reqwest::StatusCode;

use crate::common::seed::characters::ARLE;
use crate::common::{create_test_client, create_test_pool, seed};

#[tokio::test]
async fn inserts_new_alias() -> Result<(), Box<dyn std::error::Error>> {
    let (client, test_db_name) = create_test_client("N/A", "N/A").await?;

    let pool = create_test_pool(&test_db_name).await?;

    // Need a character beforehand for the foreign key
    api::characters::upsert::upsert(&pool, &ARLE.char_id, &CharacterCreate::from(ARLE.clone()))
        .await
        .unwrap();

    let response = client
        .put(format!("/aliases/{}", &seed::aliases::ARLE_ALIAS_A.alias))
        .body_json(&AliasCreate::from(seed::aliases::ARLE_ALIAS_A.clone()))
        .send()
        .await;

    response.assert_status(StatusCode::OK);
    response.assert_header("Location", format!("/aliases/{}", &seed::aliases::ARLE_ALIAS_A.alias));
    response
        .assert_json(seed::aliases::ARLE_ALIAS_A.clone().to_json())
        .await;

    Ok(())
}
