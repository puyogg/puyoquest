use chrono::{DateTime, Utc};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::cards::types::CardType;

/// Request body type for alias creation. The alias should be given as a path param.
#[derive(Debug, Clone, Serialize, Deserialize, Object, FromRow)]
pub struct AliasCreate {
    pub char_id: String,
    pub internal: bool,
    pub card_type: CardType,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object, FromRow)]
pub struct Alias {
    /// Warning: The api will return the alias as an NFKD normalized String!
    pub alias: String,
    pub char_id: String,
    pub internal: bool,
    pub card_type: CardType,
    pub updated_at: DateTime<Utc>,
}

impl From<Alias> for AliasCreate {
    fn from(a: Alias) -> Self {
        Self {
            char_id: a.char_id,
            internal: a.internal,
            card_type: a.card_type,
            updated_at: Some(a.updated_at),
        }
    }
}
