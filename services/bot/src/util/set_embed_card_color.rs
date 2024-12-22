use poise::serenity_prelude::CreateEmbed;

use super::embed_colors;

pub fn set_embed_card_color(embed: CreateEmbed, color: &Option<String>) -> CreateEmbed {
    match color {
        None => embed,
        Some(color) => match color.as_str() {
            "Red" => embed.color(embed_colors::RED),
            "Blue" => embed.color(embed_colors::BLUE),
            "Green" => embed.color(embed_colors::GREEN),
            "Yellow" => embed.color(embed_colors::YELLOW),
            "Purple" => embed.color(embed_colors::PURPLE),
            _ => embed,
        },
    }
}
