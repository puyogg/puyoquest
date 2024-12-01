use api::{
    cache::CardIconUrls,
    cards::{
        template_data::CardTemplateData,
        types::{Card, CardType},
    },
};
use chrono::{TimeZone, Utc};
use lazy_static::lazy_static;
use serde_json::json;
use unicode_normalization::UnicodeNormalization;
use urlencoding::encode;

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
            extra_power: Some(
                "https://d14ks6gfutzo56.cloudfront.net/8/87/Img201217.png".to_string()
            ),
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
        }))
        .unwrap(),
        series_name: None,
        is_lore: false,
        icons: CardIconUrls {
            normal: Some("https://d14ks6gfutzo56.cloudfront.net/0/06/Img545507.png".to_string()),
            dual_shift: Some(
                "https://d14ks6gfutzo56.cloudfront.net/a/ae/Img545507_msft.png".to_string()
            ),
            extra_power: Some(
                "https://d14ks6gfutzo56.cloudfront.net/a/ab/Img545517.png".to_string()
            ),
            extra_power_dual_shift: Some(
                "https://d14ks6gfutzo56.cloudfront.net/4/48/Img545517_msft.png".to_string()
            )
        },
        url: String::from("https://puyonexus.com/wiki/PPQ:Ally_&_Rafisol/★7"),
        cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
    pub static ref SANTA_RINGO_04: Card = Card {
        card_id: "321204".to_string(),
        char_id: "3212".to_string(),
        rarity: "4".to_string(),
        rarity_modifier: None,
        name: "Santa Ringo".to_string(),
        name_normalized: "santa ringo".nfkd().to_string(),
        jp_name: Some("サンタりんご".to_string()),
        jp_name_normalized: Some("サンタりんご".nfkd().to_string()),
        link_name: "Santa Ringo".to_string(),
        link_name_normalized: "santa ringo".nfkd().to_string(),
        card_type: CardType::Character,
        main_color: "Green".to_string(),
        side_color: None,
        wiki_template: serde_json::from_value::<CardTemplateData>(json!({
            "code": "321204",
            "rarity": "4",
            "name": "Santa Ringo",
            "ase": "RESOLVED_SKILL_TEXT",
        }))
        .unwrap(),
        series_name: None,
        is_lore: false,
        icons: CardIconUrls {
            normal: Some("https://d14ks6gfutzo56.cloudfront.net/b/b5/Img321204.png".to_string()),
            ..Default::default()
        },
        url: format!(
            "https://puyonexus.com/wiki/PPQ:{}",
            encode("Santa Ringo/★4")
        ),
        cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
    pub static ref SANTA_RINGO_05: Card = Card {
        card_id: "321205".to_string(),
        char_id: "3212".to_string(),
        rarity: "5".to_string(),
        rarity_modifier: None,
        name: "Santa Ringo".to_string(),
        name_normalized: "santa ringo".nfkd().to_string(),
        jp_name: Some("サンタりんご".to_string()),
        jp_name_normalized: Some("サンタりんご".nfkd().to_string()),
        link_name: "Santa Ringo".to_string(),
        link_name_normalized: "santa ringo".nfkd().to_string(),
        card_type: CardType::Character,
        main_color: "Green".to_string(),
        side_color: None,
        wiki_template: serde_json::from_value::<CardTemplateData>(json!({
            "code": "321205",
            "rarity": "5",
            "name": "Santa Ringo",
            "ase": "RESOLVED_SKILL_TEXT",
        }))
        .unwrap(),
        series_name: None,
        is_lore: false,
        icons: CardIconUrls {
            normal: Some("https://d14ks6gfutzo56.cloudfront.net/d/d5/Img321205.png".to_string()),
            ..Default::default()
        },
        url: format!(
            "https://puyonexus.com/wiki/PPQ:{}",
            encode("Santa Ringo/★5")
        ),
        cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
    pub static ref SANTA_RINGO_06: Card = Card {
        card_id: "321206".to_string(),
        char_id: "3212".to_string(),
        rarity: "6".to_string(),
        rarity_modifier: Some("6-1".to_string()),
        name: "Santa Ringo".to_string(),
        name_normalized: "santa ringo".nfkd().to_string(),
        jp_name: Some("サンタりんご".to_string()),
        jp_name_normalized: Some("サンタりんご".nfkd().to_string()),
        link_name: "Santa Ringo/★6-1".to_string(),
        link_name_normalized: "santa ringo/★6-1".nfkd().to_string(),
        card_type: CardType::Character,
        main_color: "Green".to_string(),
        side_color: None,
        wiki_template: serde_json::from_value::<CardTemplateData>(json!({
            "code": "321206",
            "rarity": "6",
            "name": "Santa Ringo",
            "ase": "RESOLVED_SKILL_TEXT",
        }))
        .unwrap(),
        series_name: None,
        is_lore: false,
        icons: CardIconUrls {
            normal: Some("https://d14ks6gfutzo56.cloudfront.net/c/c7/Img321206.png".to_string()),
            ..Default::default()
        },
        url: format!(
            "https://puyonexus.com/wiki/PPQ:{}",
            encode("Santa Ringo/★6-1")
        ),
        cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
    pub static ref SANTA_RINGO_6S: Card = Card {
        card_id: "321216".to_string(),
        char_id: "3212".to_string(),
        rarity: "6".to_string(),
        rarity_modifier: Some("6-2".to_string()),
        name: "Santa Ringo S".to_string(),
        name_normalized: "santa ringo s".nfkd().to_string(),
        jp_name: Some("サンタりんご・S".to_string()),
        jp_name_normalized: Some("サンタりんご・S".nfkd().to_string()),
        link_name: "Santa Ringo/★6-2".to_string(),
        link_name_normalized: "santa ringo/★6-2".nfkd().to_string(),
        card_type: CardType::Character,
        main_color: "Green".to_string(),
        side_color: Some("Red".to_string()),
        wiki_template: serde_json::from_value::<CardTemplateData>(json!({
            "code": "321216",
            "rarity": "6",
            "name": "Santa Ringo S",
            "ase": "RESOLVED_SKILL_TEXT",
        }))
        .unwrap(),
        series_name: None,
        is_lore: false,
        icons: CardIconUrls {
            normal: Some("https://d14ks6gfutzo56.cloudfront.net/b/b8/Img321216.png".to_string()),
            ..Default::default()
        },
        url: format!(
            "https://puyonexus.com/wiki/PPQ:{}",
            encode("Santa Ringo/★6-2")
        ),
        cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
    pub static ref SANTA_RINGO_MAT1: Card = Card {
        card_id: "352905".to_string(),
        char_id: "3212".to_string(),
        rarity: "5".to_string(),
        rarity_modifier: None,
        name: "Green Holy Bell (Gold)".to_string(),
        name_normalized: "green holy bell (gold)".nfkd().to_string(),
        jp_name: Some("緑の聖なるベル（金）".to_string()),
        jp_name_normalized: Some("緑の聖なるベル（金）".nfkd().to_string()),
        link_name: "Santa Ringo/Materials/★5".to_string(),
        link_name_normalized: "santa ringo/materials/★5".nfkd().to_string(),
        card_type: CardType::Material,
        main_color: "Green".to_string(),
        side_color: None,
        wiki_template: serde_json::from_value::<CardTemplateData>(json!({
            "code": "352905",
            "rarity": "5",
            "name": "Green Holy Bell (Gold)",
            "ase": "RESOLVED_SKILL_TEXT",
        }))
        .unwrap(),
        series_name: None,
        is_lore: false,
        icons: CardIconUrls {
            normal: Some("https://d14ks6gfutzo56.cloudfront.net/a/ab/Img352905.png".to_string()),
            ..Default::default()
        },
        url: format!(
            "https://puyonexus.com/wiki/PPQ:{}",
            encode("Santa Ringo/Materials/★5")
        ),
        cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
    pub static ref SANTA_RINGO_MAT2: Card = Card {
        card_id: "352906".to_string(),
        char_id: "3212".to_string(),
        rarity: "6".to_string(),
        rarity_modifier: Some("6-1".to_string()),
        name: "Snowglobe (Green)".to_string(),
        name_normalized: "snowglobe (green)".nfkd().to_string(),
        jp_name: Some("スノードーム（緑））".to_string()),
        jp_name_normalized: Some("=スノードーム（緑）".nfkd().to_string()),
        link_name: "Santa Ringo/Materials/★6-1".to_string(),
        link_name_normalized: "santa ringo/materials/★6-1".nfkd().to_string(),
        card_type: CardType::Material,
        main_color: "Green".to_string(),
        side_color: None,
        wiki_template: serde_json::from_value::<CardTemplateData>(json!({
            "code": "352906",
            "rarity": "6",
            "name": "Snowglobe (Green)",
            "ase": "RESOLVED_SKILL_TEXT",
        }))
        .unwrap(),
        series_name: None,
        is_lore: false,
        icons: CardIconUrls {
            normal: Some("https://d14ks6gfutzo56.cloudfront.net/0/00/Img352906.png".to_string()),
            ..Default::default()
        },
        url: format!(
            "https://puyonexus.com/wiki/PPQ:{}",
            encode("Santa Ringo/Materials/★6-1")
        ),
        cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
    pub static ref SANTA_RINGO_MAT3: Card = Card {
        card_id: "152806".to_string(),
        char_id: "3212".to_string(),
        rarity: "6".to_string(),
        rarity_modifier: Some("6-2".to_string()),
        name: "Snowglobe (Red)".to_string(),
        name_normalized: "snowglobe (red)".nfkd().to_string(),
        jp_name: Some("スノードーム（赤）".to_string()),
        jp_name_normalized: Some("スノードーム（赤）".nfkd().to_string()),
        link_name: "Santa Ringo/Materials/★6-2".to_string(),
        link_name_normalized: "santa ringo/materials/★6-2".nfkd().to_string(),
        card_type: CardType::Material,
        main_color: "Red".to_string(),
        side_color: None,
        wiki_template: serde_json::from_value::<CardTemplateData>(json!({
            "code": "152806",
            "rarity": "6",
            "name": "Snowglobe (Red)",
            "ase": "RESOLVED_SKILL_TEXT",
        }))
        .unwrap(),
        series_name: None,
        is_lore: false,
        icons: CardIconUrls {
            normal: Some("https://d14ks6gfutzo56.cloudfront.net/7/7f/Img152806.png".to_string()),
            ..Default::default()
        },
        url: format!(
            "https://puyonexus.com/wiki/PPQ:{}",
            encode("Santa Ringo/Materials/★6-2")
        ),
        cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
    pub static ref ON_STAGE_RINGO_07: Card = Card {
        card_id: "345507".into(),
        char_id: "3455".into(),
        rarity: "7".into(),
        rarity_modifier: None,
        name: "On-stage Ringo".into(),
        name_normalized: "on-stage ringo".nfkd().to_string(),
        jp_name: Some("オンステージのりんご".to_string()),
        jp_name_normalized: Some("オンステージのりんご".nfkd().to_string()),
        link_name: "On-stage Ringo".to_string(),
        link_name_normalized: "on-stage ringo".nfkd().to_string(),
        card_type: CardType::Character,
        main_color: "Green".to_string(),
        side_color: Some("Red".to_string()),
        wiki_template: serde_json::from_value::<CardTemplateData>(json!({
            "code": "345507",
            "rarity": "7",
            "name": "On-stage Ringo",
            "ase": "RESOLVED_SKILL_TEXT",
        }))
        .unwrap(),
        series_name: None,
        is_lore: false,
        icons: CardIconUrls::default(),
        url: format!(
            "https://puyonexus.com/wiki/PPQ:{}",
            encode("On-stage Ringo"),
        ),
        cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
    pub static ref RAINCLOUD_NINE_SIG_07: Card = Card {
        card_id: "241207".to_string(),
        char_id: "2412".to_string(),
        rarity: "7".to_string(),
        rarity_modifier: None,
        name: "Raincloud Nine Sig".to_string(),
        name_normalized: "raincloud nine sig".nfkd().to_string(),
        jp_name: Some("うきうきのシグ".to_string()),
        jp_name_normalized: Some("うきうきのシグ".nfkd().to_string()),
        link_name: "Raincloud Nine Sig".to_string(),
        link_name_normalized: "raincloud nine sig".nfkd().to_string(),
        card_type: CardType::Character,
        main_color: "Blue".to_string(),
        side_color: Some("Purple".to_string()), 
        wiki_template: serde_json::from_value::<CardTemplateData>(json!({
            "code": "241207",
            "rarity": "7",
            "name": "On-stage Ringo",
            "ase": "RESOLVED_SKILL_TEXT",
        }))
        .unwrap(),
        series_name: None,
        is_lore: false,
        icons: CardIconUrls::default(),
        url: format!(
            "https://puyonexus.com/wiki/PPQ:{}",
            encode("Raincloud Nine Sig"),
        ),
        cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
}
