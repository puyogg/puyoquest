use api::aliases::types::AliasCreate;
use api::characters::types::CharacterCreate;
use reqwest::StatusCode;

use crate::common::{create_test_client, create_test_pool};
use crate::common::{seed, IntTestResult};

#[tokio::test]
async fn deletes_an_alias() -> IntTestResult {
    let (client, test_db_name, ..) = create_test_client("N/A", "N/A").await?;

    let pool = create_test_pool(&test_db_name).await?;

    api::characters::upsert::upsert(
        &pool,
        &seed::characters::ARLE.char_id,
        &CharacterCreate::from(seed::characters::ARLE.clone()),
    )
    .await
    .unwrap();
    api::aliases::upsert::upsert(
        &pool,
        &AliasCreate::from(seed::aliases::ARLE_ALIAS_ORIGINAL.clone()),
    )
    .await
    .unwrap();
    api::aliases::upsert::upsert(
        &pool,
        &AliasCreate::from(seed::aliases::ARLE_ALIAS_A.clone()),
    )
    .await
    .unwrap();
    api::aliases::upsert::upsert(
        &pool,
        &AliasCreate::from(seed::aliases::ARLE_ALIAS_B.clone()),
    )
    .await
    .unwrap();

    let response = client
        .delete("/aliases")
        .query("name", &"idjikidjik")
        .send()
        .await;

    response.assert_status(StatusCode::OK);
    response.assert_text("1").await;
    Ok(())
}
