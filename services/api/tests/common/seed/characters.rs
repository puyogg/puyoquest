use api::characters::types::Character;
use chrono::{TimeZone, Utc};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ARLE: Character = Character {
        char_id: "2012".to_string(),
        name: Some("Arle".to_string()),
        jp_name: Some("アルル".to_string()),
        link_name: Some("Arle".to_string()),
        main_color: Some("Blue".to_string()),
        side_color: None,
        type1: Some("Attack".to_string()),
        type2: Some("Single".to_string()),
        voice_trans: Some("V2012".to_string()),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
    pub static ref SANTA_RINGO: Character = Character {
        char_id: "3212".to_string(),
        name: Some("Santa Ringo".to_string()),
        jp_name: Some("サンタりんご".to_string()),
        link_name: Some("santa ringo".to_string()),
        main_color: Some("Green".to_string()),
        side_color: Some("Red".to_string()),
        type1: Some("Balance".to_string()),
        type2: Some("Single".to_string()),
        voice_trans: Some("V1018".to_string()),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
    pub static ref LEGAMUNT: Character = Character {
        char_id: "4203".to_string(),
        name: Some("Legamünt".to_string()),
        jp_name: Some("レガムント".to_string()),
        link_name: Some("Legamünt".to_string()),
        main_color: Some("Yellow".to_string()),
        side_color: Some("Purple".to_string()),
        type1: Some("Balance".to_string()),
        type2: Some("Single".to_string()),
        voice_trans: Some("V4203".to_string()),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
    pub static ref SPACE_ECOLO: Character = Character {
        char_id: "5383".to_string(),
        name: Some("Space☆Ecolo".to_string()),
        jp_name: Some("スペース☆エコロ".to_string()),
        link_name: Some("Space Ecolo".to_string()),
        main_color: Some("Purple".to_string()),
        side_color: Some("Green".to_string()),
        type1: Some("Balance".to_string()),
        type2: Some("Single".to_string()),
        voice_trans: Some("V5383".to_string()),
        updated_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
    };
}
