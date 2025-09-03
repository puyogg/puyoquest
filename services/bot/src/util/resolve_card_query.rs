use sdk::apis::configuration::Configuration as PpqApiConfiguration;
use sdk::apis::{cards_api, characters_api};
use sdk::models::{Card, CardsAndMaterials};

use crate::util::parse_card_query::AliasAndRarityQuery;

#[derive(Debug)]
pub enum ResolveCardQueryResult {
    ExactCard(Card),
    FailedToFindCharacter,
    FailedToFindCharacterCards,
    CharacterCards(CardsAndMaterials),
}

pub async fn resolve_card_query(
    ppq_api_config: &PpqApiConfiguration,
    card_query: &AliasAndRarityQuery,
) -> ResolveCardQueryResult {
    if let Some(q) = &card_query.query {
        let card = cards_api::cards_get(&ppq_api_config, Some(&q.alias), Some(&q.rarity)).await;

        if let Ok(card) = card {
            return ResolveCardQueryResult::ExactCard(card);
        }
    }

    let characters =
        characters_api::characters_get(&ppq_api_config, Some(&card_query.fallback)).await;
    let character = match characters {
        Err(_) => {
            return ResolveCardQueryResult::FailedToFindCharacter;
        }
        Ok(characters) => {
            let character = characters.get(0).cloned();
            match character {
                None => return ResolveCardQueryResult::FailedToFindCharacter,
                Some(c) => c,
            }
        }
    };

    let cards =
        characters_api::characters_id_cards_get(&ppq_api_config, &character.char_id, Some("false"))
            .await;
    let cards = match cards {
        Ok(c) => c,
        Err(_) => return ResolveCardQueryResult::FailedToFindCharacterCards,
    };

    ResolveCardQueryResult::CharacterCards(cards)
}
