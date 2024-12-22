pub mod embed_colors;
pub mod emoji_table;
pub mod markdown;
pub mod parse_card_query;
pub mod sort_rarity;

mod set_embed_card_color;
pub use set_embed_card_color::set_embed_card_color;

mod fetch_character;
pub use fetch_character::*;
