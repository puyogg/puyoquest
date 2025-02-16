use std::collections::HashMap;

use chrono::Utc;
use futures::future::try_join_all;

use sdk::apis::cards_api;
use sdk::apis::characters_api;
use sdk::apis::configuration::Configuration;
use sdk::models::Alias;
use sdk::models::AliasCreate;
use sdk::models::{Card, CardCreate, CardType, Character, CharacterCreate};
use wiki::wiki_client::{CharacterCardIds, FetchTemplate, WikiClient};

use super::error::CardIndexerError;
use super::map_fallback_color::map_fallback_color;
use crate::parsing::parse_rarity_after_star::parse_rarity_after_star;
use crate::parsing::remove_rarity_suffix::remove_rarity_suffix;
use utils::normalize_name;

pub struct CardIndexer {
    wiki_client: WikiClient,
    api_config: Configuration,
}

pub struct CardsAndMaterials {
    pub cards: Vec<Card>,
    pub materials: Vec<Card>,
}

pub struct IndexResponses {
    pub character: Character,
    pub cards: Vec<Card>,
    pub materials: Vec<Card>,
    pub aliases: Vec<Alias>,
}

impl CardIndexer {
    pub fn new(api_host: &str) -> CardIndexer {
        CardIndexer {
            wiki_client: WikiClient::new(
                "https://puyonexus.com/mediawiki/api.php",
                "https://puyonexus.com/wiki",
            ),
            api_config: Configuration {
                base_path: api_host.to_string(),
                user_agent: None,
                client: reqwest::Client::new(),
                basic_auth: None,
                oauth_access_token: None,
                bearer_access_token: None,
                api_key: None,
            },
        }
    }

    pub async fn index_char(&self, char_id: &str) -> Result<IndexResponses, CardIndexerError> {
        let character = self.update_character(char_id).await?;
        let cards_and_materials = self.update_cards_for_character(char_id).await?;
        let aliases = self
            .update_aliases_for_character(
                char_id,
                &cards_and_materials.cards,
                &cards_and_materials.materials,
            )
            .await?;

        Ok(IndexResponses {
            character,
            cards: cards_and_materials.cards,
            materials: cards_and_materials.materials,
            aliases,
        })
    }

    pub async fn update_card(
        &self,
        char_id: Option<&str>,
        card_id: &str,
    ) -> Result<Card, CardIndexerError> {
        let char_id = match char_id {
            Some(c) => c,
            None => &card_id[..4],
        };

        let card_template = self.wiki_client.fetch_template(card_id).await?;

        let get = |key: &str| {
            card_template
                .get(key)
                .map(|c| c.as_str())
                .flatten()
                .map(|c| c.to_string())
        };

        let get_required = |key: &str| {
            let value = get(key);
            match value {
                Some(v) => Ok(v),
                None => return Err(CardIndexerError::CardMissingKeyValue(key.to_string())),
            }
        };

        let name = get_required("name")?;
        let rarity = get_required("rarity")?;
        let rarity_modifier = match get("link") {
            None => None,
            Some(link_name) => match parse_rarity_after_star(&link_name) {
                Ok(l) => l,
                Err(e) => return Err(CardIndexerError::RarityModifierParsingError(e)),
            },
        };
        let link_name = match get("link") {
            Some(l) => l,
            None => format!("{}/★{}", name.clone(), rarity.clone()),
        };

        let card_create = CardCreate {
            card_id: card_id.to_string(),
            char_id: char_id.to_string(),
            rarity,
            rarity_modifier,
            name: name.clone(),
            name_normalized: normalize_name(&name),
            jp_name: get("jpname"),
            jp_name_normalized: get("jpname").map(|j| normalize_name(&j)),
            link_name: link_name.clone(),
            link_name_normalized: normalize_name(&link_name),
            card_type: match link_name.to_lowercase().contains("materials") {
                true => CardType::Material,
                false => CardType::Character,
            },
            main_color: get_required("color")?,
            side_color: get("color2"),
            updated_at: Some(Utc::now().to_rfc3339()),
        };

        let updated_card = cards_api::cards_post(&self.api_config, card_create.clone()).await;
        let updated_card = match updated_card {
            Ok(c) => c,
            Err(e) => {
                let json_string = serde_json::to_string_pretty(&card_create)
                    .unwrap_or(format!("FAILED TO SERIALIZE: {}", card_id));
                log::warn!("Failed to upsert card data: {}", json_string);
                return Err(e)?;
            }
        };

        Ok(updated_card)
    }

    pub async fn update_character(&self, char_id: &str) -> Result<Character, CardIndexerError> {
        let char_template = self.wiki_client.fetch_template(char_id).await?;

        let get = |key: &str| {
            char_template
                .get(key)
                .map(|c| c.as_str())
                .flatten()
                .map(|c| c.to_string())
        };

        let character_create = CharacterCreate {
            name: get("name"),
            jp_name: get("jpname"),
            link_name: match get("link") {
                Some(link) => Some(link),
                None => get("name"),
            },
            main_color: match get("color") {
                Some(color) => Some(color),
                None => map_fallback_color(&char_id[..1]),
            },
            side_color: get("color2"),
            type1: get("type1"),
            type2: get("type2"),
            voice_trans: get("voicetrans"),
            updated_at: Some(Utc::now().to_rfc3339()),
        };

        log::info!(
            "Indexing character {}: {}",
            char_id,
            &character_create
                .name
                .clone()
                .unwrap_or("UNKNOWN NAME".to_string())
        );

        let updated_character =
            characters_api::characters_id_put(&self.api_config, char_id, character_create.clone())
                .await;
        let updated_character = match updated_character {
            Ok(c) => {
                log::trace!(
                    "Successfully indexed character: {} - {}",
                    c.char_id.clone(),
                    c.name.clone().unwrap_or("???".to_string())
                );
                c
            }
            Err(e) => {
                let json_string = serde_json::to_string_pretty(&character_create)
                    .unwrap_or(format!("FAILED TO SERIALIZE: {}", char_id));
                log::warn!("Failed to upsert card data: {}", json_string);
                return Err(e)?;
            }
        };

        Ok(updated_character)
    }

    pub async fn update_cards_for_character(
        &self,
        char_id: &str,
    ) -> Result<CardsAndMaterials, CardIndexerError> {
        let cards_and_materials = self.wiki_client.character_card_ids(char_id).await?;

        let cards = try_join_all(
            cards_and_materials
                .card_ids
                .iter()
                .map(|card_id| self.update_card(Some(char_id), card_id)),
        )
        .await?;

        let materials = try_join_all(
            cards_and_materials
                .material_ids
                .iter()
                .map(|card_id| self.update_card(Some(char_id), card_id)),
        )
        .await?;

        Ok(CardsAndMaterials { cards, materials })
    }

    pub async fn update_aliases_for_character(
        &self,
        char_id: &str,
        cards: &Vec<Card>,
        materials: &Vec<Card>,
    ) -> Result<Vec<Alias>, CardIndexerError> {
        let mut unique_aliases: HashMap<String, Alias> = HashMap::new();

        for (card_type, cards) in &[
            (CardType::Character, cards),
            (CardType::Material, materials),
        ] {
            for card in *cards {
                let alias_create = AliasCreate {
                    alias: card.name.to_string(),
                    char_id: char_id.to_string(),
                    internal: true,
                    card_type: *card_type,
                    updated_at: Some(Utc::now().to_rfc3339()),
                };

                let upserted_alias =
                    sdk::apis::aliases_api::aliases_post(&self.api_config, alias_create.clone())
                        .await?;

                unique_aliases.insert(upserted_alias.alias.clone(), upserted_alias);

                if let Some(jp_name) = &card.jp_name {
                    let upserted_jp_alias = sdk::apis::aliases_api::aliases_post(
                        &self.api_config,
                        AliasCreate {
                            alias: jp_name.clone(),
                            ..alias_create.clone()
                        },
                    )
                    .await?;

                    unique_aliases.insert(upserted_jp_alias.alias.clone(), upserted_jp_alias);
                }

                let link_name_no_rarity = remove_rarity_suffix(&card.link_name);
                let upserted_link_name_alias = sdk::apis::aliases_api::aliases_post(
                    &self.api_config,
                    AliasCreate {
                        alias: link_name_no_rarity,
                        ..alias_create.clone()
                    },
                )
                .await?;

                unique_aliases.insert(
                    upserted_link_name_alias.alias.clone(),
                    upserted_link_name_alias,
                );
            }
        }

        let aliases = unique_aliases.into_values().collect::<Vec<Alias>>();
        Ok(aliases)
    }
}
