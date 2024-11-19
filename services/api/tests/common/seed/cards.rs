use api::{cache::CardIconUrls, cards::{
    template_data::CardTemplateData,
    types::{Card, CardType},
}};
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
        wiki_template: serde_json::from_value::<CardTemplateData>(json!({
            "code": "201207",
            "rarity": "7",
            "name": "Arle",
            "ase": "RESOLVED_SKILL_TEXT",
        }))
        .unwrap(),
        series_name: Some("Original Puyo Puyo Series".to_string()),
        is_lore: false,
        icons: CardIconUrls {
            normal: Some("https://d14ks6gfutzo56.cloudfront.net/a/ad/Img201207.png".to_string()),
            dual_shift: None,
            extra_power: Some("https://d14ks6gfutzo56.cloudfront.net/8/87/Img201217.png".to_string()),
            extra_power_dual_shift: None,
        },
        url: String::from("https://puyonexus.com/wiki/PPQ:Arle/★7"),
        cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    pub static ref ALLY_AND_RAFISOL_07: Card = Card {
        card_id: "545507".to_string(),
        char_id: "5455".to_string(),
        rarity: "7".to_string(),
        rarity_modifier: None,
        name: "Ally & Rafisol".to_string(),
        name_normalized: "ally & rafisol".nfkd().to_string(),
        jp_name: Some("アリィ＆ラフィソル".to_string()),
        jp_name_normalized: Some("アリィ＆ラフィソル".nfkd().to_string()),
        link_name: "Ally & Rafisol".to_string(),
        link_name_normalized: "ally & rafisol".to_string(),
        card_type: CardType::Character,
        main_color: "Purple".to_string(),
        side_color: Some("Green".to_string()),
        wiki_template: serde_json::from_value::<CardTemplateData>(json!({
            "code": "545507",
            "rarity": "7",
            "name": "Ally & Rafisol",
            "ase": "RESOLVED_SKILL_TEXT",
        })).unwrap(),
        series_name: None,
        is_lore: false,
        icons: CardIconUrls {
            normal: Some("https://d14ks6gfutzo56.cloudfront.net/0/06/Img545507.png".to_string()),
            dual_shift: Some("https://d14ks6gfutzo56.cloudfront.net/a/ae/Img545507_msft.png".to_string()),
            extra_power: Some("https://d14ks6gfutzo56.cloudfront.net/a/ab/Img545517.png".to_string()),
            extra_power_dual_shift: Some("https://d14ks6gfutzo56.cloudfront.net/4/48/Img545517_msft.png".to_string())
        },
        url: String::from("https://puyonexus.com/wiki/PPQ:Ally_&_Rafisol/★7"),
        cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
}
