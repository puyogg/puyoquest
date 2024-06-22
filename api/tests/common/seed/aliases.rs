use api::{aliases::types::Alias, cards::types::CardType};
use chrono::{TimeZone, Utc};
use lazy_static::lazy_static;
use unicode_normalization::UnicodeNormalization;

lazy_static! {
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
}
