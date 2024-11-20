use poise::serenity_prelude::{
    self as serenity, ComponentInteraction, CreateActionRow, CreateButton, EditInteractionResponse,
};

use sdk::models::{Card, CardIconUrls};

use crate::commands::Error;
use crate::commands::{Context, Data};
use crate::util::embed_colors;
use crate::util::emoji_table::wiki_symbols_to_emojis;
use crate::util::markdown;

pub async fn card_embed(
    card: &Card,
    icon_type: CardIconType,
) -> Result<(serenity::CreateEmbed, Vec<CreateActionRow>), Error> {
    let embed = serenity::CreateEmbed::default()
        .title(format_card_title(card))
        .url(&card.url)
        .fields(first_page_fields(card));

    let icon_url_option = match icon_type {
        CardIconType::Normal => &card.icons.normal,
        CardIconType::DualShift => &card.icons.dual_shift,
        CardIconType::ExtraPower => &card.icons.extra_power,
        CardIconType::ExtraPowerDualShift => &card.icons.extra_power_dual_shift,
    };
    let embed = match icon_url_option {
        Some(url) => embed.thumbnail(url),
        None => embed,
    };

    let combin_category_links = format_combination_categories(&card);
    let embed = match &combin_category_links {
        Some(link_string) => embed.description(link_string),
        None => embed,
    };

    let embed = match card.main_color.as_str() {
        "Red" => embed.color(embed_colors::RED),
        "Blue" => embed.color(embed_colors::BLUE),
        "Green" => embed.color(embed_colors::GREEN),
        "Yellow" => embed.color(embed_colors::YELLOW),
        "Purple" => embed.color(embed_colors::PURPLE),
        _ => embed,
    };

    let mut components: Vec<CreateActionRow> = Vec::new();

    let icon_buttons = icon_select_row(&card.card_id, &card.icons);
    if let Some(icon_buttons) = icon_buttons {
        components.push(icon_buttons)
    };

    Ok((embed, components))
}

pub enum CardIconType {
    Normal,
    DualShift,
    ExtraPower,
    ExtraPowerDualShift,
}

pub async fn update_card_embed_icon(
    ctx: &serenity::Context,
    data: &Data,
    component_interaction: &ComponentInteraction,
) -> Result<(), Error> {
    let custom_id = &component_interaction.data.custom_id;
    let parse_result = parse_card_embed_custom_id(custom_id)?;
    println!("{:?}", parse_result);
    match parse_result {
        Some((card_id, icon_type)) => {
            let icon_type = match icon_type.as_str() {
                "main" => CardIconType::Normal,
                "dual_shift" => CardIconType::DualShift,
                "extra_power" => CardIconType::ExtraPower,
                "extra_power_dual_shift" => CardIconType::ExtraPowerDualShift,
                _ => CardIconType::Normal,
            };

            let card = sdk::apis::cards_api::cards_id_get(&data.api_config, &card_id).await?;

            let (embed, components) = card_embed(&card, icon_type).await?;
            let response = serenity::CreateInteractionResponseMessage::default().embed(embed);
            let response = if components.len() > 0 {
                response.components(components)
            } else {
                response
            };
            // let edit = component_interaction.edit_response(ctx, response).await;
            let edit = component_interaction
                .create_response(
                    ctx,
                    serenity::CreateInteractionResponse::UpdateMessage(response),
                )
                .await;
            match edit {
                Ok(e) => println!("{:?}", e),
                Err(e) => println!("{:?}", e),
            };
        }
        None => {
            component_interaction
                .create_response(
                    ctx,
                    serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::new()
                            .content("There was an error handling your card query request!"),
                    ),
                )
                .await?;
            return Ok(());
        }
    };
    Ok(())
}

lazy_static::lazy_static! {
    static ref RE_CARD_CUSTOM_ID: fancy_regex::Regex = fancy_regex::Regex::new(r"(.+):(.+):(.+)").unwrap();
}

type CardId = String;
type IconType = String;
fn parse_card_embed_custom_id(custom_id: &str) -> Result<Option<(CardId, IconType)>, Error> {
    let captures = RE_CARD_CUSTOM_ID.captures(custom_id)?;

    let data = match captures {
        Some(c) => {
            let card_id = c.get(2);
            let icon_type = c.get(3);

            match (card_id, icon_type) {
                (Some(card_id), Some(icon_type)) => {
                    Some((card_id.as_str().to_string(), icon_type.as_str().to_string()))
                }
                _ => None,
            }
        }
        None => None,
    };

    Ok(data)
}

/// https://stackoverflow.com/questions/71864137/whats-the-ideal-way-to-trim-extra-spaces-from-a-string
fn trim_whitespace(s: &str) -> String {
    // second attempt: only allocate a string
    let mut result = String::with_capacity(s.len());
    s.split_whitespace().for_each(|w| {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(w);
    });
    result
}

fn format_ls(
    tag: &str,
    english: &str,
    japanese: &Option<String>,
    level: &Option<String>,
    description: &Option<String>,
) -> (String, String, bool) {
    let english = match level {
        Some(lv) => format!("{} Lv. {}", english, lv),
        None => String::from(english),
    };

    let japanese = match (japanese, level) {
        (Some(j), Some(lv)) => format!("({} Lv. {})", j, lv),
        (Some(j), None) => format!("({})", j),
        _ => String::from(""),
    };

    let title = trim_whitespace(&format!("[{}] {} {}", tag, english, japanese));
    let description = match description {
        Some(d) => wiki_symbols_to_emojis(d),
        None => String::from("N/A"),
    };

    (title, description, false)
}

fn format_as(
    tag: &str,
    english: &str,
    japanese: &Option<String>,
    level: &Option<String>,
    color: &Option<String>,
    activation_count: &Option<String>,
    description: &Option<String>,
) -> (String, String, bool) {
    let english = match level {
        Some(lv) => format!("{} Lv. {}", english, lv),
        None => String::from(english),
    };

    let japanese = match (japanese, level) {
        (Some(j), Some(lv)) => format!("({} Lv. {})", j, lv),
        (Some(j), None) => format!("({})", j),
        _ => String::from(""),
    };

    let activation_count = match activation_count {
        Some(count) => count,
        None => "?",
    };

    let activation_puyo_emoji = match color {
        Some(c) => match c.as_ref() {
            "Red" => "<:red:429944006135382017>",
            "Blue" => "<:blue:429944006601080849>",
            "Green" => "<:green:429944006948945931>",
            "Yellow" => "<:yellow:429944006718521345>",
            "Purple" => "<:purple:429944007397736448>",
            _ => "?",
        },
        None => "?",
    };

    let title = trim_whitespace(&format!(
        "[{tag}] {english} {japanese} [{activation_puyo_emoji}×{activation_count}]"
    ));
    let description = match description {
        Some(d) => wiki_symbols_to_emojis(d),
        None => String::from("N/A"),
    };

    (title, description, false)
}

fn first_page_fields(card: &Card) -> Vec<(String, String, bool)> {
    let wiki_template = &card.wiki_template;

    let mut fields: Vec<(String, String, bool)> = Vec::new();

    let (stats_title, stats_description) = format_base_max_lv_stats(card);
    fields.push((stats_title, stats_description, false));

    if let Some(ls) = &wiki_template.ls {
        fields.push(format_ls(
            "LS",
            ls,
            &wiki_template.jpls,
            &wiki_template.lslv,
            &wiki_template.lse,
        ));
    }

    if let Some(lst) = &wiki_template.lst {
        fields.push(format_ls(
            "LS+",
            &lst,
            &wiki_template.jplst,
            &None,
            &wiki_template.lste,
        ));
    }

    if let Some(lst2) = &wiki_template.lst2 {
        fields.push(format_ls(
            "LS+",
            &lst2,
            &wiki_template.jplst2,
            &None,
            &wiki_template.lst2e,
        ));
    }

    if let Some(lst3) = &wiki_template.lst3 {
        fields.push(format_ls(
            "LS+",
            &lst3,
            &wiki_template.jplst3,
            &None,
            &wiki_template.lst3e,
        ));
    }

    if let Some(active_skill) = &wiki_template.r#as {
        fields.push(format_as(
            "AS",
            &active_skill,
            &wiki_template.jpas,
            &wiki_template.aslv,
            &wiki_template.color,
            &wiki_template.asn,
            &wiki_template.ase,
        ));
    }

    fields
}

fn format_card_title(card: &Card) -> String {
    let rarity = match &card.rarity_modifier {
        Some(rm) => match rm.as_ref() {
            "6-2" => "6S",
            _ => &card.rarity,
        },
        None => &card.rarity,
    };

    match &card.jp_name {
        Some(jp_name) => format!("{} ★{} ({})", &card.name, rarity, jp_name,),
        None => format!("{} ★{}", &card.name, rarity,),
    }
}

fn format_base_max_lv_stats(card: &Card) -> (String, String) {
    let lv = match &card.wiki_template.maxlv {
        Some(max_lv) => max_lv,
        None => "?",
    };
    let title = format!("Base Lv. {lv} Stats");

    let hp = &card
        .wiki_template
        .hpmax
        .clone()
        .unwrap_or(String::from("?"));
    let atk = &card
        .wiki_template
        .atkmax
        .clone()
        .unwrap_or(String::from("?"));
    let rcv = &card
        .wiki_template
        .rcvmax
        .clone()
        .unwrap_or(String::from("?"));
    let cost = &card.wiki_template.cost.clone().unwrap_or(String::from("?"));
    let card_type = match (&card.wiki_template.type1, &card.wiki_template.type2) {
        (Some(type1), Some(type2)) => {
            if type2 == "Mass" {
                format!("{type1}/Mass")
            } else {
                String::from(type1)
            }
        }
        (Some(type1), None) => String::from(type1),
        _ => String::from("?"),
    };

    let description = format!(
        r#"HP: {hp}　ATK: {atk}　RCV: {rcv}
Cost: {cost}　Type: {card_type}"#
    );

    (title, description)
}

fn format_combination_categories(card: &Card) -> Option<String> {
    let combins = vec![
        &card.wiki_template.combin1,
        &card.wiki_template.combin2,
        &card.wiki_template.combin3,
        &card.wiki_template.combin4,
        &card.wiki_template.combin5,
        &card.wiki_template.combin6,
    ]
    .iter()
    .filter_map(|combin_name| match combin_name {
        Some(s) => Some(markdown::combination_link(s)),
        None => None,
    })
    .collect::<Vec<String>>()
    .join(" ");

    if combins.len() > 0 {
        Some(combins)
    } else {
        None
    }
}

fn icon_select_row(
    card_id: &str,
    card_icon_urls: &CardIconUrls,
) -> Option<serenity::CreateActionRow> {
    let mut buttons: Vec<serenity::CreateButton> = Vec::new();

    if card_icon_urls.normal.is_some() {
        buttons.push(serenity::CreateButton::new(&format!("card:{card_id}:normal")).label("REG"));
    }
    if card_icon_urls.dual_shift.is_some() {
        buttons
            .push(serenity::CreateButton::new(&format!("card:{card_id}:dual_shift")).label("DS"));
    }
    if card_icon_urls.extra_power.is_some() {
        buttons
            .push(serenity::CreateButton::new(&format!("card:{card_id}:extra_power")).label("EP"));
    }
    if card_icon_urls.extra_power_dual_shift.is_some() {
        buttons.push(
            serenity::CreateButton::new(&format!("card:{card_id}:extra_power_dual_shift"))
                .label("EP+DS"),
        );
    }

    if buttons.len() > 0 {
        Some(serenity::CreateActionRow::Buttons(buttons))
    } else {
        None
    }
}
