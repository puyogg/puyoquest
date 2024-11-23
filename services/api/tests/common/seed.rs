use api::{
    aliases::types::AliasCreate, cards::types::CardCreate, characters::types::CharacterCreate,
};
use sqlx::PgPool;

pub mod aliases;
pub mod cards;
pub mod characters;

#[allow(dead_code)]
pub async fn seed_arle(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let character = CharacterCreate::from(characters::ARLE.clone());
    let card = CardCreate::from(cards::ARLE_07.clone());
    let alias = AliasCreate::from(aliases::ARLE_ALIAS_ORIGINAL.clone());

    api::characters::upsert::upsert(&pool, &characters::ARLE.char_id, &character)
        .await
        .unwrap();
    api::cards::upsert::upsert(&pool, &card).await.unwrap();
    api::aliases::upsert(&pool, &aliases::ARLE_ALIAS_ORIGINAL.alias, &alias)
        .await
        .unwrap();

    Ok(())
}

#[allow(dead_code)]
pub async fn seed_santa_ringo(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let character = CharacterCreate::from(characters::SANTA_RINGO.clone());
    api::characters::upsert::upsert(&pool, &characters::SANTA_RINGO.char_id, &character).await.unwrap();

    let cards = vec![
        cards::SANTA_RINGO_04.clone(),
        cards::SANTA_RINGO_05.clone(),
        cards::SANTA_RINGO_06.clone(),
        cards::SANTA_RINGO_6S.clone(),
        cards::SANTA_RINGO_MAT1.clone(),
        cards::SANTA_RINGO_MAT2.clone(),
        cards::SANTA_RINGO_MAT3.clone(),
    ];
    for card in cards {
        let card = CardCreate::from(card);
        api::cards::upsert::upsert(&pool, &card).await.unwrap();
    }

    let alias = AliasCreate::from(aliases::SANTA_RINGO_XMAS.clone());
    api::aliases::upsert(&pool, &aliases::SANTA_RINGO.alias, &alias).await.unwrap();

    Ok(())
}
