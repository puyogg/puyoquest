use api::{
    aliases::types::{Alias, AliasCreate},
    characters::types::{Character, CharacterCreate},
};
use futures::{future::join_all, TryFutureExt};
use poem_openapi::types::ToJSON;
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::common::IntTestResult;
use crate::common::{create_test_client, create_test_pool, seed};
use urlencoding::encode;

async fn seed(pool: &PgPool, data: &[(Character, Alias)]) -> IntTestResult {
    let creates: Vec<(String, CharacterCreate, AliasCreate)> = data
        .into_iter()
        .map(|(c, a)| {
            (
                c.char_id.clone(),
                CharacterCreate::from(c.clone()),
                AliasCreate::from(a.clone()),
            )
        })
        .collect();

    let create_futures = creates.iter().map(|(char_id, c, a)| {
        api::characters::upsert(pool, &char_id, &c)
            .and_then(|_| async { api::aliases::upsert(pool, a).await })
    });
    join_all(create_futures).await;
    Ok(())
}

#[tokio::test]
async fn lists_aliases_by_char_id() -> Result<(), Box<dyn std::error::Error>> {
    let (client, test_db_name, ..) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    let arle_create = CharacterCreate::from(seed::characters::ARLE.clone());
    api::characters::upsert::upsert(&pool, &seed::characters::ARLE.char_id, &arle_create)
        .await
        .unwrap();

    let alias_creates: Vec<AliasCreate> = vec![
        AliasCreate::from(seed::aliases::ARLE_ALIAS_ORIGINAL.clone()),
        AliasCreate::from(seed::aliases::ARLE_ALIAS_A.clone()),
        AliasCreate::from(seed::aliases::ARLE_ALIAS_B.clone()),
    ];
    let alias_futures = alias_creates
        .iter()
        .map(|ac| api::aliases::upsert(&pool, &ac));
    join_all(alias_futures).await;

    let mut response = client.get("/aliases?char_id=2012").send().await;

    response.assert_status(StatusCode::OK);
    let aliases: Vec<Alias> = response.0.take_body().into_json().await.unwrap();

    let arle_original: Option<&Alias> = aliases
        .iter()
        .find(|a| a.alias == seed::aliases::ARLE_ALIAS_ORIGINAL.alias);
    assert!(arle_original.is_some());

    let arle_a: Option<&Alias> = aliases
        .iter()
        .find(|a| a.alias == seed::aliases::ARLE_ALIAS_A.alias);
    assert!(arle_a.is_some());

    let arle_b: Option<&Alias> = aliases
        .iter()
        .find(|a| a.alias == seed::aliases::ARLE_ALIAS_B.alias);
    assert!(arle_b.is_some());

    assert_eq!(aliases.len(), 3);

    Ok(())
}

#[tokio::test]
async fn lists_fuzzy_matched_aliases_legamunt() -> IntTestResult {
    let (client, test_db_name, ..) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    let data = &[
        (seed::characters::MARS.clone(), seed::aliases::MARS.clone()),
        (seed::characters::YURI.clone(), seed::aliases::YURI.clone()),
        (
            seed::characters::HARTMANN.clone(),
            seed::aliases::HARTMANN.clone(),
        ),
        (
            seed::characters::EMILIA.clone(),
            seed::aliases::EMILIA.clone(),
        ),
        (
            seed::characters::VIOLA.clone(),
            seed::aliases::VIOLA.clone(),
        ),
        (
            seed::characters::LEGAMUNT.clone(),
            seed::aliases::LEGAMUNT_ORIGINAL.clone(),
        ),
    ];
    seed(&pool, data).await?;

    let response = client
        .get(format!("/aliases?name={}", encode("Legamünt")))
        .send()
        .await;
    response
        .assert_json(
            &[
                seed::aliases::LEGAMUNT_ORIGINAL.clone(),
                seed::aliases::HARTMANN.clone(),
            ]
            .to_json()
            .unwrap(),
        )
        .await;

    Ok(())
}
