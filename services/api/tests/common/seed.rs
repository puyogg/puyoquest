use api::{
    aliases::types::AliasCreate, cards::types::CardCreate, characters::types::CharacterCreate,
};
use sqlx::PgPool;

pub mod aliases;
pub mod cards;
pub mod characters;

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
