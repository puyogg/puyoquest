use api::cards::types::{Card, CardType};
use chrono::{TimeZone, Utc};
use lazy_static::lazy_static;
use serde_json::json;
use unicode_normalization::UnicodeNormalization;

lazy_static! {
    pub static ref ARLE_07: Card = Card {
        card_id: "201207".to_string(),
        char_id: "2012".to_string(),
        rarity: "7".to_string(),
        rarity_modifier: None,
        name: "Arle".to_string(),
        name_normalized: "arle".nfkd().to_string(),
        jp_name: Some("アルル".to_string()),
        jp_name_normalized: Some("アルル".to_string()),
        link_name: "Arle".to_string(),
        link_name_normalized: "arle".to_string(),
        card_type: CardType::Character,
        main_color: "Blue".to_string(),
        side_color: None,
        wiki_template: Some(json!(r#"{ "as": "Active Skill", "ls": "Leader Skill" }"#)),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap()
    };
}
