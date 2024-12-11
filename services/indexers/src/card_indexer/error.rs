use sdk::apis::{cards_api::CardsPostError, characters_api::CharactersIdPutError};
use wiki::wiki_client::{CardAndMaterialIdsError, FetchTemplateError};

#[derive(Debug)]
pub enum IndexerError {
    InvalidCharOrCardId(String),

    FetchTemplateError(FetchTemplateError),
    FetchCardIdsError(CardAndMaterialIdsError),
    SerdeJsonError(serde_json::Error),

    UpdateCharacterError(sdk::apis::Error<CharactersIdPutError>),
    UpdateCardError(sdk::apis::Error<CardsPostError>),
    CardMissingKeyValue(String),
}

impl std::error::Error for IndexerError {}
impl std::fmt::Display for IndexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexerError::InvalidCharOrCardId(id) => write!(f, "Invalid id: {}", &id),
            IndexerError::FetchTemplateError(fetch_template_error) => fetch_template_error.fmt(f),
            IndexerError::FetchCardIdsError(card_and_material_ids_error) => {
                card_and_material_ids_error.fmt(f)
            }
            IndexerError::SerdeJsonError(error) => error.fmt(f),
            IndexerError::UpdateCharacterError(error) => error.fmt(f),
            IndexerError::UpdateCardError(error) => error.fmt(f),
            IndexerError::CardMissingKeyValue(key) => {
                write!(f, "Missing key value from card template: {}", key)
            }
        }
    }
}

impl From<serde_json::Error> for IndexerError {
    fn from(value: serde_json::Error) -> Self {
        IndexerError::SerdeJsonError(value)
    }
}
