use api::characters::types::Character;
use chrono::{TimeZone, Utc};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ARLE: Character = Character {
        char_id: "2012".to_string(),
        name: "Arle".to_string(),
        jp_name: "アルル".to_string(),
        link_name: "Arle".to_string(),
        main_color: Some("Blue".to_string()),
        side_color: None,
        type1: Some("Attack".to_string()),
        type2: Some("Single".to_string()),
        voice_trans: Some("V2012".to_string()),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
}
