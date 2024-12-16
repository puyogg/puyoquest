use api::aliases::types::{Alias, AliasCreate};
use api::characters::types::CharacterCreate;
use poem_openapi::types::ToJSON;
use reqwest::StatusCode;

use crate::common::seed::characters::{ARLE, LEGAMUNT, SANTA_RINGO};
use crate::common::{create_test_client, create_test_pool, seed};

#[tokio::test]
async fn inserts_new_alias() -> Result<(), Box<dyn std::error::Error>> {
    let (client, test_db_name, ..) = create_test_client("N/A", "N/A").await?;

    let pool = create_test_pool(&test_db_name).await?;

    // Need a character beforehand for the foreign key
    api::characters::upsert::upsert(&pool, &ARLE.char_id, &CharacterCreate::from(ARLE.clone()))
        .await
        .unwrap();

    let response = client
        .post("/aliases")
        .body_json(&AliasCreate::from(seed::aliases::ARLE_ALIAS_A.clone()))
        .send()
        .await;

    response.assert_status(StatusCode::OK);
    response
        .assert_json(seed::aliases::ARLE_ALIAS_A.clone().to_json())
        .await;

    Ok(())
}

#[tokio::test]
async fn updates_alias() -> Result<(), Box<dyn std::error::Error>> {
    let (client, test_db_name, ..) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    api::characters::upsert::upsert(&pool, &ARLE.char_id, &CharacterCreate::from(ARLE.clone()))
        .await
        .unwrap();

    let mut response1 = client
        .post("/aliases")
        .body_json(&AliasCreate::from(seed::aliases::ARLE_ALIAS_A.clone()))
        .send()
        .await;
    let alias_before: Alias = response1.0.take_body().into_json().await.unwrap();
    assert_eq!(alias_before.internal, false);

    let modified_arle_a = AliasCreate {
        internal: true,
        updated_at: None,
        ..AliasCreate::from(seed::aliases::ARLE_ALIAS_A.clone())
    };
    let mut response2 = client
        .post("/aliases")
        .body_json(&modified_arle_a)
        .send()
        .await;
    let alias_after: Alias = response2.0.take_body().into_json().await.unwrap();
    assert_eq!(alias_after.internal, true);

    assert!(alias_before.updated_at < alias_after.updated_at);

    Ok(())
}

#[tokio::test]
async fn inserts_alias_with_middle_space() -> Result<(), Box<dyn std::error::Error>> {
    let (client, test_db_name, ..) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    api::characters::upsert::upsert(
        &pool,
        &SANTA_RINGO.char_id,
        &CharacterCreate::from(SANTA_RINGO.clone()),
    )
    .await
    .unwrap();

    let response = client
        .post("/aliases")
        .body_json(&AliasCreate::from(seed::aliases::SANTA_RINGO_XMAS.clone()))
        .send()
        .await;

    response.assert_status(StatusCode::OK);
    response
        .assert_json(seed::aliases::SANTA_RINGO_XMAS.to_json())
        .await;

    Ok(())
}

#[tokio::test]
async fn inserts_alias_with_japanese_name() -> Result<(), Box<dyn std::error::Error>> {
    let (client, test_db_name, ..) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    api::characters::upsert::upsert(
        &pool,
        &SANTA_RINGO.char_id,
        &CharacterCreate::from(SANTA_RINGO.clone()),
    )
    .await
    .unwrap();

    let response = client
        .post("/aliases")
        .body_json(&AliasCreate::from(seed::aliases::SANTA_RINGO_JP.clone()))
        .send()
        .await;

    response.assert_status(StatusCode::OK);
    response
        .assert_json(seed::aliases::SANTA_RINGO_JP.to_json())
        .await;

    Ok(())
}

#[tokio::test]
async fn normalizes_alias_name_on_insert() -> Result<(), Box<dyn std::error::Error>> {
    let (client, test_db_name, ..) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    api::characters::upsert::upsert(
        &pool,
        &LEGAMUNT.char_id,
        &CharacterCreate::from(LEGAMUNT.clone()),
    )
    .await
    .unwrap();

    let ac = AliasCreate {
        alias: "Legamünt".to_string(),
        ..AliasCreate::from(seed::aliases::LEGAMUNT_ORIGINAL.clone())
    };

    let response = client.post("/aliases").body_json(&ac).send().await;

    response.assert_status(StatusCode::OK);
    response
        .assert_json(seed::aliases::LEGAMUNT_ORIGINAL.to_json())
        .await;

    Ok(())
}
