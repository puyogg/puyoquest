use std::collections::HashMap;

use super::error::IndexerError;
use super::util;
use chrono::Utc;
use futures::future::try_join_all;
use sdk::apis::cards_api;
use sdk::apis::characters_api;
use sdk::apis::configuration::Configuration;
use sdk::models::Alias;
use sdk::models::AliasCreate;
use sdk::models::{Card, CardCreate, CardType, Character, CharacterCreate};
use wiki::wiki_client;
use wiki::wiki_client::{CharacterCardIds, FetchTemplate, WikiClient};

pub struct Indexer {
    wiki_client: WikiClient,
    char_id: String,
    api_config: sdk::apis::configuration::Configuration,
}

impl Indexer {
    pub fn new(api_host: &str, _api_key: Option<String>, char_id: &str) -> Indexer {
        Indexer {
            wiki_client: WikiClient::new(
                "https://puyonexus.com/mediawiki/api.php",
                "https://puyonexus.com/wiki",
            ),
            char_id: char_id.to_string(),
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

    pub async fn from_id(
        api_host: &str,
        _api_key: Option<String>,
        id: &str,
    ) -> Result<Indexer, IndexerError> {
        if id.len() == 4 {
            Ok(Indexer::new(api_host, _api_key, id))
        } else if id.len() == 6 {
            let client = wiki_client::WikiClient::new(
                "https://puyonexus.com/mediawiki/api.php",
                "https://puyonexus.com/wiki",
            );

            let maybe_card_template = client
                .fetch_template(id)
                .await
                .map_err(IndexerError::FetchTemplateError)?;

            let char_value = maybe_card_template.get("char");

            let indexer = match char_value {
                Some(c) => match c.as_str() {
                    Some(c) => Indexer::new(api_host, _api_key, c),
                    None => return Err(IndexerError::InvalidCharOrCardId(id.to_string())),
                },
                None => Indexer::new(api_host, _api_key, &id[..4]),
            };

            Ok(indexer)
        } else {
            Err(IndexerError::InvalidCharOrCardId(id.to_string()))
        }
    }

    pub async fn update_character_and_cards(&self) -> Result<(), IndexerError> {
        let _ = self.update_character().await?;
        let (cards, materials) = self.update_cards().await?;
        let _ = self
            .create_internal_aliases(&self.char_id, cards, materials)
            .await?;
        Ok(())
    }

    pub async fn update_character(&self) -> Result<Character, IndexerError> {
        let char_template = self
            .wiki_client
            .fetch_template(&self.char_id)
            .await
            .map_err(IndexerError::FetchTemplateError)?;

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
                None => util::map_fallback_color(&self.char_id[..1]),
            },
            side_color: get("color2"),
            type1: get("type1"),
            type2: get("type2"),
            voice_trans: get("voicetrans"),
            updated_at: Some(Utc::now().to_rfc3339()),
        };

        println!(
            "Indexing character {}: {}",
            &self.char_id,
            &character_create
                .name
                .clone()
                .unwrap_or("UNKNOWN NAME".to_string())
        );

        let updated_character = characters_api::characters_id_put(
            &self.api_config,
            &self.char_id,
            character_create.clone(),
        )
        .await
        .map_err(IndexerError::UpdateCharacterError)?;

        // println!(
        //     "Updated char_id {}: {}",
        //     &self.char_id,
        //     &character_create.name.unwrap_or("MISSING NAME".to_string())
        // );

        Ok(updated_character)
    }

    async fn update_cards(&self) -> Result<(Vec<Card>, Vec<Card>), IndexerError> {
        let cards_and_materials = self
            .wiki_client
            .character_card_ids(&self.char_id)
            .await
            .map_err(IndexerError::FetchCardIdsError)?;

        // TODO: Batch these
        let cards = try_join_all(
            cards_and_materials
                .card_ids
                .iter()
                .map(|card_id| self.update_card(&card_id)),
        )
        .await?;

        let materials = try_join_all(
            cards_and_materials
                .material_ids
                .iter()
                .map(|material_id| self.update_card(&material_id)),
        )
        .await?;

        Ok((cards, materials))
    }

    pub async fn update_card(&self, card_id: &str) -> Result<Card, IndexerError> {
        let card_template = self
            .wiki_client
            .fetch_template(card_id)
            .await
            .map_err(IndexerError::FetchTemplateError)?;

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
                None => return Err(IndexerError::CardMissingKeyValue(key.to_string())),
            }
        };

        let name = get_required("name")?;
        let link_name = match get("link") {
            Some(l) => l,
            None => name.clone(),
        };

        let card_create = CardCreate {
            card_id: card_id.to_string(),
            char_id: self.char_id.to_string(),
            rarity: get_required("rarity")?,
            rarity_modifier: util::parse_rarity_modifier(&link_name),
            name: name.clone(),
            name_normalized: utils::normalize_name(&name),
            jp_name: get("jpname"),
            jp_name_normalized: get("jpname").map(|j| utils::normalize_name(&j)),
            link_name: link_name.clone(),
            link_name_normalized: utils::normalize_name(&link_name),
            card_type: match link_name.to_lowercase().contains("materials") {
                true => CardType::Material,
                false => CardType::Character,
            },
            main_color: get_required("color")?,
            side_color: get("color2"),
            updated_at: Some(Utc::now().to_rfc3339()),
        };

        let updated_card = cards_api::cards_post(&self.api_config, card_create)
            .await
            .map_err(IndexerError::UpdateCardError)?;

        Ok(updated_card)
    }

    pub async fn create_internal_aliases(
        &self,
        char_id: &str,
        cards: Vec<Card>,
        materials: Vec<Card>,
    ) -> Result<Vec<Alias>, IndexerError> {
        let mut unique_aliases: HashMap<String, Alias> = HashMap::new();

        for (card_type, cards) in &[
            (CardType::Character, cards),
            (CardType::Material, materials),
        ] {
            for card in cards {
                let alias_create = AliasCreate {
                    alias: card.name.to_string(),
                    char_id: char_id.to_string(),
                    internal: true,
                    card_type: *card_type,
                    updated_at: Some(Utc::now().to_rfc3339()),
                };
                let a_name =
                    sdk::apis::aliases_api::aliases_post(&self.api_config, alias_create.clone())
                        .await
                        .map_err(IndexerError::UpdateAliasError)?;
                unique_aliases.insert(a_name.alias.clone(), a_name);

                if let Some(jp_name) = &card.jp_name {
                    let a_jp_name = sdk::apis::aliases_api::aliases_post(
                        &self.api_config,
                        AliasCreate {
                            alias: jp_name.clone(),
                            ..alias_create.clone()
                        },
                    )
                    .await
                    .map_err(IndexerError::UpdateAliasError)?;
                    unique_aliases.insert(a_jp_name.alias.clone(), a_jp_name);
                }

                let a_link_name = sdk::apis::aliases_api::aliases_post(
                    &self.api_config,
                    AliasCreate {
                        alias: card.link_name.clone(),
                        ..alias_create
                    },
                )
                .await
                .map_err(IndexerError::UpdateAliasError)?;
                unique_aliases.insert(a_link_name.alias.clone(), a_link_name);
            }
        }

        let aliases = unique_aliases.into_values().collect::<Vec<Alias>>();
        // println!("Upserted internal aliases: {:#?}", aliases);

        Ok(aliases)
    }
}
