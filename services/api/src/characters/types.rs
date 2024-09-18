use chrono::{DateTime, Utc};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Object, FromRow, Serialize)]
pub struct CharacterCreate {
    pub name: Option<String>,
    pub jp_name: Option<String>,
    pub link_name: Option<String>,
    pub main_color: Option<String>,
    pub side_color: Option<String>,
    pub type1: Option<String>,
    pub type2: Option<String>,
    pub voice_trans: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Object, FromRow, Serialize, Deserialize)]
pub struct Character {
    pub char_id: String,
    pub name: Option<String>,
    pub jp_name: Option<String>,
    pub link_name: Option<String>,
    pub main_color: Option<String>,
    pub side_color: Option<String>,
    pub type1: Option<String>,
    pub type2: Option<String>,
    pub voice_trans: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl From<Character> for CharacterCreate {
    fn from(c: Character) -> Self {
        Self {
            name: c.name,
            jp_name: c.jp_name,
            link_name: c.link_name,
            main_color: c.main_color,
            side_color: c.side_color,
            type1: c.type1,
            type2: c.type2,
            voice_trans: c.voice_trans,
            updated_at: Some(c.updated_at),
        }
    }
}
