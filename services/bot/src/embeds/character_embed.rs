use poise::serenity_prelude::CreateSelectMenuOption;
use poise::serenity_prelude as serenity;
use serenity::{CreateActionRow, CreateEmbed};
use urlencoding::encode;

use sdk::models::{Card, CardsAndMaterials, Character};

use crate::util::markdown;
use crate::util::sort_rarity::sort_rarity;
use crate::util::embed_colors;

pub fn character_embed(
    character: &Character,
    cards_and_materials: &CardsAndMaterials,
) -> (CreateEmbed, Vec<CreateActionRow>){
    let embed = serenity::CreateEmbed::default().title(format_character_title(character));

    let embed = match &character.link_name {
        None => embed,
        Some(link_name) => embed.url(format!(
            "https://puyonexus.com/wiki/PPQ:{}",
            encode(link_name)
        )),
    };

    let embed = match &character.main_color {
        None => embed,
        Some(main_color) => match main_color.as_str() {
            "Red" => embed.color(embed_colors::RED),
            "Blue" => embed.color(embed_colors::BLUE),
            "Green" => embed.color(embed_colors::GREEN),
            "Yellow" => embed.color(embed_colors::YELLOW),
            "Purple" => embed.color(embed_colors::PURPLE),
            _ => embed,
        }
    };

    let cards_and_materials = cards_and_materials.clone();
    let mut cards = cards_and_materials.cards;
    cards.sort_by(|a, b| {
        sort_rarity(
            (&a.rarity, &a.rarity_modifier),
            (&b.rarity, &b.rarity_modifier),
        )
    });
    let mut materials = cards_and_materials.materials;
    materials.sort_by(|a, b| {
        sort_rarity(
            (&a.rarity, &a.rarity_modifier),
            (&b.rarity, &b.rarity_modifier),
        )
    });

    let embed = if cards.len() > 0 {
        let rarity_links = markdown::rarity_link_list(&cards);
        embed.field("Rarities", rarity_links, false)
    } else {
        embed
    };

    let embed = if materials.len() > 0 {
        let material_links = markdown::material_link_list(&materials);
        embed.field("Character-specific materials", material_links, false)
    } else {
        embed
    };

    let embed = match cards.last() {
        Some(c) => match &c.icons.normal {
            Some(url) => embed.thumbnail(url),
            None => embed,
        },
        None => embed,
    };

    let mut select_menu_options: Vec<CreateSelectMenuOption> = Vec::new();
    let cards_and_mats = [&cards[..], &materials[..]].concat();
    for card in cards_and_mats {
        let label = format_select_menu_option(&card);
        let value = &card.card_id;
        select_menu_options.push(CreateSelectMenuOption::new(label, value));
    }

    let char_id = &character.char_id;
    let select_menu = serenity::CreateSelectMenu::new(
        format!("card:update::{char_id}:"),
        serenity::CreateSelectMenuKind::String {
            options: select_menu_options,
        },
    )
    .placeholder("Request card:");

    let action_row = serenity::CreateActionRow::SelectMenu(select_menu);

    (embed, vec![action_row])
}

fn format_character_title(character: &Character) -> String {
    match (&character.name, &character.jp_name) {
        (Some(name), Some(jp_name)) => format!("{} ({})", name, jp_name),
        (Some(name), None) => format!("{}", name),
        (None, Some(jp_name)) => format!("? ({})", jp_name),
        _ => String::from("?"),
    }
}

fn format_select_menu_option(card: &Card) -> String {
    let rarity = match &card.rarity_modifier {
        Some(rm) => rm,
        None => &card.rarity,
    };
    let name = &card.name;

    match &card.jp_name {
        Some(jp_name) => format!("[★{rarity}] {name} ({jp_name})"),
        None => format!("[★{rarity}] {name}"),
    }
}
