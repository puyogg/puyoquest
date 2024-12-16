use api::{aliases::types::Alias, cards::types::CardType};
use chrono::{TimeZone, Utc};
use lazy_static::lazy_static;
use unicode_normalization::UnicodeNormalization;

lazy_static! {
    // Arle
    pub static ref ARLE_ALIAS_ORIGINAL: Alias = Alias {
        alias: "arle".nfkd().to_string(),
        char_id: "2012".to_string(),
        internal: true,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    pub static ref ARLE_ALIAS_A: Alias = Alias {
        alias: "arley".nfkd().to_string(),
        char_id: "2012".to_string(),
        internal: false,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    pub static ref ARLE_ALIAS_B: Alias = Alias {
        alias: "idjikidjik".nfkd().to_string(),
        char_id: "2012".to_string(),
        internal: false,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    // Santa Ringo
    pub static ref SANTA_RINGO: Alias = Alias {
        alias: "santa ringo".nfkd().to_string(),
        char_id: "3212".to_string(),
        internal: true,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    pub static ref SANTA_RINGO_XMAS: Alias = Alias {
        alias: "xmas ringo".nfkd().to_string(),
        char_id: "3212".to_string(),
        internal: false,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    pub static ref SANTA_RINGO_JP: Alias = Alias {
        alias: utils::normalize_name("サンタりんご"),
        char_id: "3212".to_string(),
        internal: false,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    pub static ref LEGAMUNT_ORIGINAL: Alias = Alias {
        alias: "legamunt".to_string(),
        char_id: "4203".to_string(),
        internal: true,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    pub static ref SPACE_ECOLO_JP: Alias = Alias {
        alias: utils::normalize_name("スペース☆エコロ"),
        char_id: "5383".to_string(),
        internal: true,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    pub static ref MARS: Alias = Alias {
        alias: "mars".to_string(),
        char_id: "1089".to_string(),
        internal: true,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    pub static ref YURI: Alias = Alias {
        alias: "yuri".to_string(),
        char_id: "2089".to_string(),
        internal: true,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    pub static ref HARTMANN: Alias = Alias {
        alias: "hartmann".to_string(),
        char_id: "3089".to_string(),
        internal: true,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    pub static ref EMILIA: Alias = Alias {
        alias: "emilia".to_string(),
        char_id: "4089".to_string(),
        internal: true,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };

    pub static ref VIOLA: Alias = Alias {
        alias: "viola".to_string(),
        char_id: "5089".to_string(),
        internal: true,
        card_type: CardType::Character,
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
}
