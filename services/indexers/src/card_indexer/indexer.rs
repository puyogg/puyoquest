use super::error::IndexerError;
use super::util;
use chrono::Utc;
use sdk::apis::cards_api;
use sdk::apis::characters_api;
use sdk::apis::configuration::Configuration;
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
        let _ = self.update_cards().await?;
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

    async fn update_cards(&self) -> Result<(), IndexerError> {
        let cards_and_materials = self
            .wiki_client
            .character_card_ids(&self.char_id)
            .await
            .map_err(IndexerError::FetchCardIdsError)?;

        // TODO: Batch these
        for card_id in &cards_and_materials.card_ids {
            let _card = self.update_card(&card_id).await?;
        }

        for mat_id in &cards_and_materials.material_ids {
            let _mat = self.update_card(&mat_id).await?;
        }

        Ok(())
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
}
