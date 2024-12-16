use poise::serenity_prelude::{self as serenity, CreateActionRow, CreateEmbed, CreateSelectMenu, CreateSelectMenuOption};
use sdk::models::{Card, CardFullArtUrls};
use super::format_card_title;

use crate::{interaction_router::component::button::{card_handler::card_nav_button, character_handler::character_nav_button, lore_handler::lore_nav_button}, util::embed_colors};

pub enum FullArtType {
    NormalLeft,
    NormalRight,
    ExtraPowerLeft,
    ExtraPowerRight,
    Ss,
    ExtraPowerSs,
    DualShiftLeft,
    DualShiftRight,
    ExtraPowerDualShift,
}

impl ToString for FullArtType {
    fn to_string(&self) -> String {
        match self {
            FullArtType::NormalLeft => "normal_left".to_string(),
            FullArtType::NormalRight => "normal_right".to_string(),
            FullArtType::ExtraPowerLeft => "extra_power_left".to_string(),
            FullArtType::ExtraPowerRight => "extra_power_right".to_string(),
            FullArtType::Ss => "ss".to_string(),
            FullArtType::ExtraPowerSs => "extra_power_ss".to_string(),
            FullArtType::DualShiftLeft => "dual_shift_left".to_string(),
            FullArtType::DualShiftRight => "dual_shift_right".to_string(),
            FullArtType::ExtraPowerDualShift => "extra_power_dual_shift".to_string(),
        }
    }
}

impl From<&str> for FullArtType {
    fn from(value: &str) -> Self {
        match value {
            "normal_left" => FullArtType::NormalLeft,
            "normal_right" => FullArtType::NormalRight,
            "extra_power_left" => FullArtType::ExtraPowerLeft,
            "extra_power_right" => FullArtType::ExtraPowerRight,
            "ss" => FullArtType::Ss,
            "extra_power_ss" => FullArtType::ExtraPowerSs,
            "dual_shift_left" => FullArtType::DualShiftLeft,
            "dual_shift_right" => FullArtType::DualShiftRight,
            "extra_power_dual_shift" => FullArtType::ExtraPowerDualShift,
            _ => FullArtType::NormalLeft,
        }
    }
}

impl FullArtType {
    fn select_url<'a>(full_art: &'a CardFullArtUrls, full_art_type: &'a FullArtType) -> &'a Option<String> {
        match full_art_type {
            FullArtType::NormalLeft => &full_art.normal_left,
            FullArtType::NormalRight => &full_art.normal_right,
            FullArtType::ExtraPowerLeft => &full_art.extra_power_left,
            FullArtType::ExtraPowerRight => &full_art.extra_power_right,
            FullArtType::Ss => &full_art.ss,
            FullArtType::ExtraPowerSs => &full_art.extra_power_ss,
            FullArtType::DualShiftLeft => &full_art.dual_shift_left,
            FullArtType::DualShiftRight => &full_art.dual_shift_right,
            FullArtType::ExtraPowerDualShift => &full_art.extra_power_dual_shift,
        }
    }

    fn friendly_name(&self) -> String {
        match self {
            FullArtType::NormalLeft => "Left".to_string(),
            FullArtType::NormalRight => "Right".to_string(),
            FullArtType::ExtraPowerLeft => "Extra Power (Left)".to_string(),
            FullArtType::ExtraPowerRight => "Extra Power (Right)".to_string(),
            FullArtType::Ss => "Full Power / Cross Ability".to_string(),
            FullArtType::ExtraPowerSs => "Extra Power FP/CA".to_string(),
            FullArtType::DualShiftLeft => "Dual Shift (Left)".to_string(),
            FullArtType::DualShiftRight => "Dual Shift (Right)".to_string(),
            FullArtType::ExtraPowerDualShift => "Extra Power Dual Shift".to_string(),
        }
    }
}

pub fn full_art_embed(
    card: &Card,
    full_art: &CardFullArtUrls,
    full_art_type: FullArtType,
) -> (CreateEmbed, Vec<CreateActionRow>) {
    let embed = serenity::CreateEmbed::default()
        .title(format_full_art_title(card, &full_art_type))
        .url(&card.url);

    let embed = match card.main_color.as_str() {
        "Red" => embed.color(embed_colors::RED),
        "Blue" => embed.color(embed_colors::BLUE),
        "Green" => embed.color(embed_colors::GREEN),
        "Yellow" => embed.color(embed_colors::YELLOW),
        "Purple" => embed.color(embed_colors::PURPLE),
        _ => embed,
    };

    let full_art_url = FullArtType::select_url(full_art, &full_art_type);
    let embed = match full_art_url {
        Some(url) => embed.image(url),
        None => embed,
    };

    let mut components: Vec<CreateActionRow> = Vec::new();

    let navigation = embed_navigation(card);
    components.push(navigation);

    let variant_select_menu = type_selector(&card.card_id, full_art);
    if let Some(menu) = variant_select_menu {
        components.push(menu);
    }
    
    (embed, components)
}

fn format_full_art_title(
    card: &Card,
    full_art_type: &FullArtType,
) -> String {
    format!(
        "{} | {}",
        format_card_title(card),
        full_art_type.friendly_name(),
    )
}

fn type_selector(
    card_id: &str,
    full_art: &CardFullArtUrls,
) -> Option<CreateActionRow> {
    let mut select_menu_options: Vec<CreateSelectMenuOption> = Vec::new();

    for (url, full_art_type) in [
        (&full_art.normal_left, FullArtType::NormalLeft),
        (&full_art.normal_right, FullArtType::NormalRight),
        (&full_art.extra_power_left, FullArtType::ExtraPowerLeft),
        (&full_art.extra_power_right, FullArtType::ExtraPowerRight),
        (&full_art.ss, FullArtType::Ss),
        (&full_art.extra_power_ss, FullArtType::ExtraPowerSs),
        (&full_art.dual_shift_left, FullArtType::DualShiftLeft),
        (&full_art.dual_shift_right, FullArtType::DualShiftRight),
        (&full_art.extra_power_dual_shift, FullArtType::ExtraPowerDualShift),
    ] {
        if let Some(_url) = url {
            let option = CreateSelectMenuOption::new(
                full_art_type.friendly_name(),
                full_art_type.to_string(),
            );
            select_menu_options.push(option);
        }
    }

    if select_menu_options.len() == 0 {
        return None
    }

    let select_menu = CreateSelectMenu::new(
        format!("full_art:update::{card_id}:"),
        serenity::CreateSelectMenuKind::String { options: select_menu_options },
    ).placeholder("Select variant:");
    
    Some(CreateActionRow::SelectMenu(select_menu))
}

fn embed_navigation(card: &Card) -> CreateActionRow {
    let buttons: Vec<serenity::CreateButton> = vec![
        character_nav_button(&card.char_id),
        card_nav_button(&card.card_id, super::CardIconType::Normal),
        lore_nav_button(&card.card_id),
    ];

    serenity::CreateActionRow::Buttons(buttons)
}
