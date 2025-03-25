use poise::serenity_prelude::CreateEmbed;
use sdk::models::{Card, CardFullArtUrls};

use crate::util::embed_colors;

pub fn ntc_embed(card: &Card, full_art: &CardFullArtUrls) -> CreateEmbed {
    let embed = CreateEmbed::default();

    let embed = match embed_colors::color_from_str(&card.main_color) {
        Some(c) => embed.color(c),
        None => embed,
    };

    let full_art_url = &full_art.normal_left;
    let embed = match full_art_url {
        Some(url) => embed.image(url),
        None => embed,
    };

    embed
}
