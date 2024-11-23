use async_trait::async_trait;
use fancy_regex::Regex;
use serde::{Deserialize, Serialize};

use super::{FetchTemplate, FetchTemplateError};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardAndMaterialIds {
    pub card_ids: Vec<String>,
    pub material_ids: Vec<String>,
}

#[derive(Debug)]
pub enum CardAndMaterialIdsError {
    Reqwest(reqwest::Error),
    FetchTemplateError(FetchTemplateError),
    RegexError(fancy_regex::Error),
}

impl std::error::Error for CardAndMaterialIdsError {}
impl std::fmt::Display for CardAndMaterialIdsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(error) => error.fmt(f),
            Self::FetchTemplateError(error) => error.fmt(f),
            Self::RegexError(error) => error.fmt(f),
        }
    }
}

impl From<reqwest::Error> for CardAndMaterialIdsError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

#[async_trait]
pub trait CharacterCardIds {
    async fn character_card_ids(
        &self,
        char_id: &str,
    ) -> Result<CardAndMaterialIds, CardAndMaterialIdsError>;
}

lazy_static::lazy_static! {
    static ref RE_CARD_KEY: Regex = Regex::new(r"^card\d+").unwrap();
    static ref RE_MAT_KEY: Regex = Regex::new(r"^mat\d+").unwrap();
}

#[async_trait]
impl CharacterCardIds for super::WikiClient {
    async fn character_card_ids(
        &self,
        char_id: &str,
    ) -> Result<CardAndMaterialIds, CardAndMaterialIdsError> {
        let template = self
            .fetch_template(char_id)
            .await
            .map_err(|e| CardAndMaterialIdsError::FetchTemplateError(e))?;

        let mut card_ids: Vec<String> = Vec::new();
        let mut material_ids: Vec<String> = Vec::new();

        let obj = match template.as_object() {
            None => {
                return Ok(CardAndMaterialIds {
                    card_ids,
                    material_ids,
                })
            }
            Some(t) => t,
        };

        for key in obj.keys() {
            let is_card_key = RE_CARD_KEY
                .is_match(key.as_str())
                .map_err(|e| CardAndMaterialIdsError::RegexError(e))?;

            if is_card_key {
                let value = obj.get(key);
                if let Some(v) = value {
                    if let Some(s) = v.as_str() {
                        card_ids.push(s.to_string());
                    }
                }
                continue;
            }

            let is_mat_key = RE_MAT_KEY
                .is_match(key.as_str())
                .map_err(|e| CardAndMaterialIdsError::RegexError(e))?;

            if is_mat_key {
                let value = obj.get(key);
                if let Some(v) = value {
                    if let Some(s) = v.as_str() {
                        material_ids.push(s.to_string());
                    }
                }
                continue;
            }
        }

        Ok(CardAndMaterialIds {
            card_ids,
            material_ids,
        })
    }
}
