use chrono::{DateTime, Utc};
use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use urlencoding::encode;
use wiki::wiki_client::WikiClient;

use crate::{aws::s3::S3BackupClient, cache::{self, CardIconUrls, RedisClient}, config::ApiConfig, env_config::ENV};

use super::template_data::CardTemplateData;

#[derive(Enum, Clone, Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
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

#[derive(Debug, Clone, Object, Serialize, Deserialize, PartialEq, Eq)]
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
    pub wiki_template: CardTemplateData,
    pub series_name: Option<String>,
    pub is_lore: bool,
    pub icons: CardIconUrls,
    pub cached_at: DateTime<Utc>,

    // Calculated fields
    pub url: String,

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
            link_name: c.link_name.clone(),
            link_name_normalized: c.link_name_normalized,
            card_type: c.card_type,
            main_color: c.main_color,
            side_color: c.side_color,
            wiki_template: CardTemplateData::default(),
            updated_at: c.updated_at.unwrap_or_default(),
            series_name: None,
            is_lore: false,
            icons: CardIconUrls::default(),
            cached_at: Utc::now(),
            url: format!("https://puyonexus.com/wiki/PPQ:{}", encode(&c.link_name)),
        }
    }
}

impl Card {
    pub async fn upgrade_card_db(
        api_config: &ApiConfig,
        redis_client: &RedisClient,
        wiki_client: &WikiClient,
        s3_client: &S3BackupClient,
        card_db: CardDb,
    ) -> Result<Card, poem::Error> {
        let image_base_url = api_config.get_image_cache_domain().await?;
        let (wiki_template, series_data, card_icons, ..) = futures::try_join!(
            cache::card_template_data(redis_client, wiki_client, &card_db.card_id,),
            cache::character_series_data(redis_client, wiki_client, &card_db.char_id, &card_db.link_name),
            cache::card_icons(redis_client, wiki_client, s3_client, &image_base_url, &card_db),
        )?;

        let card_with_extras = Card {
            wiki_template,
            series_name: match &series_data {
                None => None,
                Some(s) => Some(String::from(&s.0)),
            },
            is_lore: match &series_data {
                None => false,
                Some(s) => s.1,
            },
            icons: card_icons,
            url: format!("{}/PPQ:{}", &*ENV.pn_wiki_api_url, encode(&card_db.link_name)),
            ..Card::from(card_db)
        };

        Ok(card_with_extras)
    }
}
