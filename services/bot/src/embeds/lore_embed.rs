use poise::serenity_prelude::{self as serenity, CreateActionRow};
use sdk::models::Card;
use sdk::models::Lore;
use sdk::models::MonologueLine;
use super::CardIconType;
use super::format_card_title;

use crate::interaction_router::component::button::card_handler::card_nav_button;
use crate::interaction_router::component::button::character_handler::character_nav_button;
use crate::util::embed_colors;

pub fn lore_embed(
    card: &Card,
    lore: &Lore,
) -> (serenity::CreateEmbed, Vec<CreateActionRow>) {
    let embed = serenity::CreateEmbed::default()
        .title(format_card_title(card))
        .url(&card.url);

    let embed = match &card.icons.normal {
        None => embed,
        Some(url) => embed.thumbnail(url),
    };

    let embed = match card.main_color.as_str() {
        "Red" => embed.color(embed_colors::RED),
        "Blue" => embed.color(embed_colors::BLUE),
        "Green" => embed.color(embed_colors::GREEN),
        "Yellow" => embed.color(embed_colors::YELLOW),
        "Purple" => embed.color(embed_colors::PURPLE),
        _ => embed,
    };

    let embed = match format_description(lore) {
        Some(d) => embed.description(d),
        None => embed,
    };

    let embed = embed.field(
        "Flavor Text",
        format_flavor_text(lore),
        false,
    );

    let embed = embed.field(
        "Monologue Lines",
        format_monologue_lines(lore),
        false,
    );

    let mut components: Vec<CreateActionRow> = Vec::new();

    let navigation = embed_navigation(card);
    components.push(navigation);

    (embed, components)
}

fn format_description(lore: &Lore) -> Option<String> {
    let description = match (&lore.translator, &lore.editor) {
        (Some(t), Some(e)) => format!("Translator: {t} | Editor: {e}"),
        (Some(t), None) => format!("Translator: {t}"),
        (None, Some(e)) => format!("Editor: {e}"),
        (None, None) => return None,
    };

    Some(description)
}

fn format_flavor_text(lore: &Lore) -> String {
    let jp = match &lore.flavor_text_jp {
        Some(t) => t.clone(),
        None => "N/A".to_string(),
    };

    let en = match &lore.flavor_text_en {
        Some(t) => t.clone(),
        None => "N/A".to_string(),
    };

    format!(r#"```{jp}```{en}"#)
}

fn format_monologue_lines(lore: &Lore) -> String {
    let line_blocks = lore.monologue_lines
        .iter()
        .map(|MonologueLine {jp, en}| {
            let jp = jp.clone().unwrap_or("N/A".to_string());
            let en = en.clone().unwrap_or("N/A".to_string());

            format!(r#"```{jp}```{en}"#)
        })
        .collect::<Vec<String>>();
    
    line_blocks.join("\n")
}

fn embed_navigation(card: &Card) -> serenity::CreateActionRow {
    let buttons: Vec<serenity::CreateButton> = vec![
        character_nav_button(&card.char_id),
        card_nav_button(&card.card_id, CardIconType::Normal),
        // serenity::CreateButton::new("tbd_full_art").label("Art"),
    ];

    serenity::CreateActionRow::Buttons(buttons)
}
