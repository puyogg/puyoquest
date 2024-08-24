use api::{
    aliases::types::AliasCreate, characters::types::CharacterCreate,
    util::url_encode_nfc::url_encode_nfc,
};
use poem_openapi::types::ToJSON;
use reqwest::StatusCode;

use crate::common::seed::aliases;
use crate::common::{
    create_test_client, create_test_pool,
    seed::{
        self,
        characters::{LEGAMUNT, SANTA_RINGO, SPACE_ECOLO},
    },
    IntTestResult,
};

#[tokio::test]
async fn finds_single_alias() -> IntTestResult {
    let (client, test_db_name) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    api::characters::upsert::upsert(
        &pool,
        &SANTA_RINGO.char_id,
        &CharacterCreate::from(SANTA_RINGO.clone()),
    )
    .await
    .unwrap();

    api::aliases::upsert(
        &pool,
        "サンタりんご",
        &AliasCreate::from(aliases::SANTA_RINGO_JP.clone()),
    )
    .await
    .unwrap();

    let response = client
        .get(format!("/aliases/{}", url_encode_nfc("サンタりんご")))
        .send()
        .await;

    response.assert_status(StatusCode::OK);
    response
        .assert_json(aliases::SANTA_RINGO_JP.to_json())
        .await;

    Ok(())
}

#[tokio::test]
async fn finds_alias_ignoring_special_chars() -> IntTestResult {
    let (client, test_db_name) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    api::characters::upsert::upsert(
        &pool,
        &SPACE_ECOLO.char_id,
        &CharacterCreate::from(SPACE_ECOLO.clone()),
    )
    .await
    .unwrap();

    api::aliases::upsert(
        &pool,
        "スペース☆エコロ",
        &AliasCreate::from(aliases::SPACE_ECOLO_JP.clone()),
    )
    .await
    .unwrap();

    let response = client
        .get(format!("/aliases/{}", url_encode_nfc("スペース☆エコロ")))
        .send()
        .await;

    response.assert_status_is_ok();
    response
        .assert_json(seed::aliases::SPACE_ECOLO_JP.to_json())
        .await;

    Ok(())
}

#[tokio::test]
async fn finds_alias_ignoring_accents() -> IntTestResult {
    let (client, test_db_name) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    api::characters::upsert::upsert(
        &pool,
        &LEGAMUNT.char_id,
        &CharacterCreate::from(LEGAMUNT.clone()),
    )
    .await
    .unwrap();

    api::aliases::upsert(
        &pool,
        "legamünt",
        &AliasCreate::from(aliases::LEGAMUNT_ORIGINAL.clone()),
    )
    .await
    .unwrap();

    let response = client
        .get(format!("/aliases/{}", url_encode_nfc("legamünt")))
        .send()
        .await;

    response.assert_status_is_ok();
    response
        .assert_json(seed::aliases::LEGAMUNT_ORIGINAL.to_json())
        .await;

    Ok(())
}

#[tokio::test]
async fn not_found() -> IntTestResult {
    let (client, _) = create_test_client("N/A", "N/A").await?;

    let response = client
        .get(format!("/aliases/{}", "DanteFromTheDevilMayCrySeries"))
        .send()
        .await;

    response.assert_status(StatusCode::NOT_FOUND);
    response
        .assert_text("Alias dantefromthedevilmaycryseries not found")
        .await;

    Ok(())
}
