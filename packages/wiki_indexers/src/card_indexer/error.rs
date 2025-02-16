use crate::parsing::error::ParsingError;
use sdk::apis::{
    aliases_api::AliasesPostError, cards_api::CardsPostError, characters_api::CharactersIdPutError,
};
use wiki::wiki_client::{CardAndMaterialIdsError, FetchTemplateError};

#[derive(Debug)]
pub enum CardIndexerError {
    InvalidCharOrCardId(String),

    FetchTemplateError(FetchTemplateError),
    FetchCardIdsError(CardAndMaterialIdsError),
    SerdeJsonError(serde_json::Error),

    UpdateCharacterError(sdk::apis::Error<CharactersIdPutError>),
    UpdateCardError(sdk::apis::Error<CardsPostError>),
    UpdateAliasError(sdk::apis::Error<AliasesPostError>),

    CardMissingKeyValue(String),
    RarityModifierParsingError(ParsingError),
}

impl std::error::Error for CardIndexerError {}
impl std::fmt::Display for CardIndexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardIndexerError::InvalidCharOrCardId(id) => write!(f, "Invalid id: {}", &id),
            CardIndexerError::FetchTemplateError(fetch_template_error) => {
                fetch_template_error.fmt(f)
            }
            CardIndexerError::FetchCardIdsError(card_and_material_ids_error) => {
                card_and_material_ids_error.fmt(f)
            }
            CardIndexerError::SerdeJsonError(error) => error.fmt(f),
            CardIndexerError::UpdateCharacterError(error) => error.fmt(f),
            CardIndexerError::UpdateCardError(error) => error.fmt(f),
            CardIndexerError::UpdateAliasError(error) => error.fmt(f),
            CardIndexerError::CardMissingKeyValue(key) => {
                write!(f, "Missing key value from card template: {}", key)
            }
            CardIndexerError::RarityModifierParsingError(parsing_error) => parsing_error.fmt(f),
        }
    }
}

impl From<serde_json::Error> for CardIndexerError {
    fn from(value: serde_json::Error) -> Self {
        CardIndexerError::SerdeJsonError(value)
    }
}

impl From<FetchTemplateError> for CardIndexerError {
    fn from(value: FetchTemplateError) -> Self {
        CardIndexerError::FetchTemplateError(value)
    }
}

impl From<sdk::apis::Error<CharactersIdPutError>> for CardIndexerError {
    fn from(value: sdk::apis::Error<CharactersIdPutError>) -> Self {
        CardIndexerError::UpdateCharacterError(value)
    }
}

impl From<CardAndMaterialIdsError> for CardIndexerError {
    fn from(value: CardAndMaterialIdsError) -> Self {
        CardIndexerError::FetchCardIdsError(value)
    }
}

impl From<sdk::apis::Error<CardsPostError>> for CardIndexerError {
    fn from(value: sdk::apis::Error<CardsPostError>) -> Self {
        CardIndexerError::UpdateCardError(value)
    }
}

impl From<sdk::apis::Error<AliasesPostError>> for CardIndexerError {
    fn from(value: sdk::apis::Error<AliasesPostError>) -> Self {
        CardIndexerError::UpdateAliasError(value)
    }
}
