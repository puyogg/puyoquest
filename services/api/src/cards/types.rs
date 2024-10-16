use chrono::{DateTime, Utc};
use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::template_data::CardTemplateData;

#[derive(Enum, Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "card_type", rename_all = "lowercase")]
#[oai(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    Character,
    Material,
}

#[derive(Debug, Clone, Object, FromRow, Serialize)]
pub struct CardCreate {
    pub card_id: String,
    /// Foreign key to the character table
    pub char_id: String,
    pub rarity: String,
    /// 6-1, 6-2 (6S)
    pub rarity_modifier: Option<String>,
    pub name: String,
    /// NFKD normalized with special characters removed
    pub name_normalized: String,
    pub jp_name: Option<String>,
    pub jp_name_normalized: Option<String>,
    pub link_name: String,
    pub link_name_normalized: String,
    pub card_type: CardType,
    pub main_color: String,
    pub side_color: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CardDb {
    pub card_id: String,
    /// Foreign key to the character table
    pub char_id: String,
    pub rarity: String,
    /// 6-1, 6-2 (6S)
    pub rarity_modifier: Option<String>,
    pub name: String,
    /// NFKD normalized with special characters removed
    pub name_normalized: String,
    pub jp_name: Option<String>,
    pub jp_name_normalized: Option<String>,
    pub link_name: String,
    pub link_name_normalized: String,
    pub card_type: CardType,
    pub main_color: String,
    pub side_color: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Object, Serialize, Deserialize)]
pub struct Card {
    pub card_id: String,
    /// Foreign key to the character table
    pub char_id: String,
    pub rarity: String,
    /// 6-1, 6-2 (6S)
    pub rarity_modifier: Option<String>,
    pub name: String,
    /// NFKD normalized with special characters removed
    pub name_normalized: String,
    pub jp_name: Option<String>,
    pub jp_name_normalized: Option<String>,
    pub link_name: String,
    pub link_name_normalized: String,
    pub card_type: CardType,
    pub main_color: String,
    pub side_color: Option<String>,

    // These fields come from the cache. They're not saved in the db
    pub wiki_template: Option<CardTemplateData>,
    // pub icon_url: Option<String>,

    pub updated_at: DateTime<Utc>,
}

impl From<CardCreate> for CardDb {
    fn from(c: CardCreate) -> Self {
        Self {
            card_id: c.card_id,
            char_id: c.char_id,
            rarity: c.rarity,
            rarity_modifier: c.rarity_modifier,
            name: c.name,
            name_normalized: c.name_normalized,
            jp_name: c.jp_name,
            jp_name_normalized: c.jp_name_normalized,
            link_name: c.link_name,
            link_name_normalized: c.link_name_normalized,
            card_type: c.card_type,
            main_color: c.main_color,
            side_color: c.side_color,
            updated_at: c.updated_at,
        }
    }
}

impl From<Card> for CardCreate {
    fn from(c: Card) -> Self {
        Self {
            card_id: c.card_id,
            char_id: c.char_id,
            rarity: c.rarity,
            rarity_modifier: c.rarity_modifier,
            name: c.name,
            name_normalized: c.name_normalized,
            jp_name: c.jp_name,
            jp_name_normalized: c.jp_name_normalized,
            link_name: c.link_name,
            link_name_normalized: c.link_name_normalized,
            card_type: c.card_type,
            main_color: c.main_color,
            side_color: c.side_color,
            updated_at: Some(c.updated_at),
        }
    }
}

impl From<CardDb> for Card {
    fn from(c: CardDb) -> Self {
        Self {
            card_id: c.card_id,
            char_id: c.char_id,
            rarity: c.rarity,
            rarity_modifier: c.rarity_modifier,
            name: c.name,
            name_normalized: c.name_normalized,
            jp_name: c.jp_name,
            jp_name_normalized: c.jp_name_normalized,
            link_name: c.link_name,
            link_name_normalized: c.link_name_normalized,
            card_type: c.card_type,
            main_color: c.main_color,
            side_color: c.side_color,
            wiki_template: None,
            updated_at: c.updated_at.unwrap_or_default(),
        }
    }
}
