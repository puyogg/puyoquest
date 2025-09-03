use sdk::{apis::configuration::Configuration as PpqApiConfiguration, models::Card};
use bot::util::{parse_card_query::{self, parse_alias_and_rarity, AliasAndRarity, AliasAndRarityQuery}, resolve_card_query, ResolveCardQueryResult};

use crate::common::get_ppq_api_config;

#[tokio::test]
async fn exact_card() -> Result<(), Box<dyn std::error::Error>> {
    let api_config = get_ppq_api_config();

    let query = AliasAndRarityQuery {
        query: Some(AliasAndRarity { alias: "arle".to_string(), rarity: "7".to_string() }),
        fallback: "arle".to_string(),
    };

    let result = resolve_card_query(&api_config, &query).await;

    match result {
        ResolveCardQueryResult::ExactCard(card) => {
            assert_eq!(card.card_id, "201207");
        },
        other => {
            println!("{:?}", other);
            panic!("Wrong resolve variant for ResolveCardQueryResult");
        }
    }

    Ok(())
}

#[tokio::test]
async fn failed_to_find_character() -> Result<(), Box<dyn std::error::Error>> {
    let api_config = get_ppq_api_config();

    let query = AliasAndRarityQuery {
        query: Some(AliasAndRarity { alias: "asdf".to_string(), rarity: "6".to_string() }),
        fallback: "asdf".to_string(),
    };

    let result = resolve_card_query(&api_config, &query).await;

    std::matches!(result, ResolveCardQueryResult::FailedToFindCharacter);

    Ok(())
}

#[tokio::test]
async fn finds_character_cards() -> Result<(), Box<dyn std::error::Error>> {
    let api_config = get_ppq_api_config();

    let query = parse_alias_and_rarity("Schezo ver. Division 24");
    let q = query.clone().query.unwrap();
    assert_eq!(q.alias, "Schezo ver. Division");
    assert_eq!(q.rarity, "24");
    assert_eq!(query.fallback, "Schezo ver. Division 24");

    let result = resolve_card_query(&api_config, &query).await;

    match result {
        ResolveCardQueryResult::CharacterCards(cards) => {
            assert!(cards.cards.iter().any(|c| c.card_id == "526306"));
            assert!(cards.cards.iter().any(|c| c.card_id == "526307"));
        },
        other => {
            println!("{:?}", other);
            panic!("Wrong resolve variant for ResolveCardQueryResult");
        }
    }

    Ok(())
}