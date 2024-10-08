use api::{
    aliases::types::{Alias, AliasCreate},
    characters::types::CharacterCreate,
};
use futures::future::join_all;
use reqwest::StatusCode;

use crate::common::seed::aliases::{ARLE_ALIAS_A, ARLE_ALIAS_B, ARLE_ALIAS_ORIGINAL};
use crate::common::{create_test_client, create_test_pool, seed};

#[tokio::test]
async fn lists_aliases_by_char_id() -> Result<(), Box<dyn std::error::Error>> {
    let (client, test_db_name, ..) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    let arle_create = CharacterCreate::from(seed::characters::ARLE.clone());
    api::characters::upsert::upsert(&pool, &seed::characters::ARLE.char_id, &arle_create)
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
    let alias_futures = alias_creates
        .iter()
        .map(|(name, ac)| api::aliases::upsert(&pool, &name, &ac));
    join_all(alias_futures).await;

    let mut response = client.get("/characters/2012/aliases").send().await;

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
