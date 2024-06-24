use api::aliases::types::AliasCreate;
use api::characters::types::{Character, CharacterCreate};
use futures::future::join_all;
use poem_openapi::types::ToJSON;
use reqwest::StatusCode;

use crate::common::{create_test_client, create_test_pool, IntTestResult};
use crate::common::seed::aliases::{ARLE_ALIAS_A, ARLE_ALIAS_B, ARLE_ALIAS_ORIGINAL};
use crate::common::seed;

#[tokio::test]
async fn finds_character_by_alias() -> IntTestResult {
    let (client, test_db_name) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    api::characters::upsert(
        &pool,
        &seed::characters::ARLE.char_id,
        &CharacterCreate::from(seed::characters::ARLE.clone()),
    )
    .await
    .unwrap();

    let alias_creates: Vec<(&str, AliasCreate)> = vec![
        (
            &ARLE_ALIAS_ORIGINAL.alias,
            AliasCreate::from(seed::aliases::ARLE_ALIAS_ORIGINAL.clone()),
        ),
        (
            &ARLE_ALIAS_A.alias,
            AliasCreate::from(seed::aliases::ARLE_ALIAS_A.clone()),
        ),
        (
            &ARLE_ALIAS_B.alias,
            AliasCreate::from(seed::aliases::ARLE_ALIAS_B.clone()),
        ),
    ];
    let alias_futures = alias_creates.iter().map(|(name, ac)| {
        api::aliases::upsert(&pool, &name, &ac)
    });
    join_all(alias_futures).await;

    let response = client
        .get("/characters?alias=idjikidjik")
        .send()
        .await;

    let single_match = vec![seed::characters::ARLE.to_json()];

    response.assert_status_is_ok();
    response.assert_json(serde_json::to_value(single_match).unwrap()).await;

    Ok(())
}

#[tokio::test]
async fn returns_empty_vec_if_no_match() -> IntTestResult {
    let (client, test_db_name) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    api::characters::upsert(
        &pool,
        &seed::characters::ARLE.char_id,
        &CharacterCreate::from(seed::characters::ARLE.clone()),
    )
    .await
    .unwrap();

    let alias_creates: Vec<(&str, AliasCreate)> = vec![
        (
            &ARLE_ALIAS_ORIGINAL.alias,
            AliasCreate::from(seed::aliases::ARLE_ALIAS_ORIGINAL.clone()),
        ),
        (
            &ARLE_ALIAS_A.alias,
            AliasCreate::from(seed::aliases::ARLE_ALIAS_A.clone()),
        ),
        (
            &ARLE_ALIAS_B.alias,
            AliasCreate::from(seed::aliases::ARLE_ALIAS_B.clone()),
        ),
    ];
    let alias_futures = alias_creates.iter().map(|(name, ac)| {
        api::aliases::upsert(&pool, &name, &ac)
    });
    join_all(alias_futures).await;

    let response = client
        .get("/characters?alias=nobodywhoareyou")
        .send()
        .await;

    let empty_vec: Vec<Character> = vec![];
    response.assert_status_is_ok();
    response.assert_json(serde_json::to_value(empty_vec).unwrap()).await;

    Ok(())
}

#[tokio::test]
async fn bad_request_if_missing_alias_name() -> IntTestResult {
    let (client, _) = create_test_client("N/A", "N/A").await?;

    let response = client
        .get("/characters")
        .send()
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);
    response.assert_text("Missing query parameter: alias").await;

    Ok(())
}