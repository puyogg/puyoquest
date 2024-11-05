use crate::wiki_client::fetch_template::FetchTemplateError;
use async_trait::async_trait;
use fancy_regex::Regex;

use super::FetchTemplate;

lazy_static::lazy_static! {
    static ref SERIES_CODE: Regex = Regex::new(r"S\d\d\d").unwrap();
}

#[derive(Debug)]
pub enum FetchCharacterSeriesError {
    CharacterPageNotFound(reqwest::Error),
    CharacterPageTextResolutionError(reqwest::Error),
    SeriesTemplateParsingError(fancy_regex::Error),
    FetchTemplateError(FetchTemplateError),
}

impl std::error::Error for FetchCharacterSeriesError {}
impl std::fmt::Display for FetchCharacterSeriesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchCharacterSeriesError::CharacterPageNotFound(e) => e.fmt(f),
            FetchCharacterSeriesError::CharacterPageTextResolutionError(e) => e.fmt(f),
            FetchCharacterSeriesError::SeriesTemplateParsingError(e) => e.fmt(f),
            FetchCharacterSeriesError::FetchTemplateError(e) => e.fmt(f),
        }
    }
}

type SeriesName = String;
type IsLore = bool;

#[async_trait]
pub trait FetchCharacterSeries {
    async fn fetch_character_series(
        &self,
        char_id: &str,
        link_name: &str,
    ) -> Result<Option<(SeriesName, IsLore)>, FetchCharacterSeriesError>;
}

#[async_trait]
impl FetchCharacterSeries for super::WikiClient {
    async fn fetch_character_series(
        &self,
        char_id: &str,
        link_name: &str,
    ) -> Result<Option<(SeriesName, IsLore)>, FetchCharacterSeriesError> {
        let link_name_url = format!("{}/PPQ:{}?action=raw", &self.base_url, &link_name);
        let raw_template = self
            .client
            .get(link_name_url)
            .send()
            .await
            .map_err(|e| FetchCharacterSeriesError::CharacterPageNotFound(e))?
            .text()
            .await
            .map_err(|e| FetchCharacterSeriesError::CharacterPageTextResolutionError(e))?;

        let series_match = SERIES_CODE
            .find(&raw_template)
            .map_err(FetchCharacterSeriesError::SeriesTemplateParsingError)?;

        match series_match {
            None => Ok(None),
            Some(series_code) => {
                let series_code = series_code.as_str();
                let series_template = self
                    .fetch_template(series_code)
                    .await
                    .map_err(FetchCharacterSeriesError::FetchTemplateError)?;

                let series_template_map = series_template.as_object();
                let series_template_map = match series_template_map {
                    None => return Ok(None),
                    Some(t) => t,
                };

                let series_name = series_template_map
                    .get("name")
                    .and_then(|s| s.as_str())
                    .map(|s| s.to_string());

                let is_lore = series_template_map
                    .iter()
                    .find(|(key, template_char_id)| {
                        let template_char_id = template_char_id.as_str();
                        match template_char_id {
                            None => false,
                            Some(t) => {
                                let is_lore_key = key.contains("lore");
                                let char_id_match = t == char_id;
                                char_id_match && is_lore_key
                            }
                        }
                    })
                    .is_some();

                let result = match (series_name, is_lore) {
                    (Some(s), true) => Some((s, true)),
                    (Some(s), false) => Some((s, false)),
                    _ => None,
                };

                Ok(result)
            }
        }
    }
}
