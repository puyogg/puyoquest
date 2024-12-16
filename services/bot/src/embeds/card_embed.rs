use fancy_regex::Regex;
use poise::serenity_prelude::{self as serenity, CreateActionRow};

use sdk::models::Card;

use crate::interaction_router::component::button::character_handler::character_nav_button;
use crate::interaction_router::component::button::full_art_handler::full_art_nav_button;
use crate::interaction_router::component::button::lore_handler::lore_nav_button;
use crate::util::embed_colors;
use crate::util::emoji_table::wiki_symbols_to_emojis;
use crate::util::markdown::{self, series_link};

pub enum CardIconType {
    Normal,
    DualShift,
    ExtraPower,
    ExtraPowerDualShift,
}

pub fn card_embed(
    card: &Card,
    icon_type: CardIconType,
) -> (serenity::CreateEmbed, Vec<CreateActionRow>) {
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

    let description = format_embed_description(card);
    let embed = match &description {
        Some(d) => embed.description(d),
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

    let navigation = embed_navigation(card);
    components.push(navigation);

    // let icon_buttons = icon_select_row(&card.card_id, &card.icons);
    // if let Some(icon_buttons) = icon_buttons {
    //     components.push(icon_buttons)
    // };

    (embed, components)
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

fn format_embed_description(card: &Card) -> Option<String> {
    let series = match &card.series_name {
        None => None,
        Some(s) => Some(series_link(&s)),
    };

    let combinations = format_combination_categories(card);

    let description = match (series, combinations) {
        (Some(s), Some(c)) => Some(format!(
            r#"{s}
{c}"#
        )),
        (Some(s), None) => Some(s),
        (None, Some(c)) => Some(c),
        (None, None) => None,
    };

    description
}

lazy_static::lazy_static! {
    static ref RE_CITATION: Regex = Regex::new(r"\[\d\]").unwrap();
    static ref RE_FIELD_EFFECT: Regex = Regex::new(r"\[\[Field Effect\](.*?)\]").unwrap();
}

fn format_description_markdown(desc: &str) -> String {
    let field_effect_captures = RE_FIELD_EFFECT.captures(desc);
    let field_effect_value = match field_effect_captures {
        Err(_) => None,
        Ok(c) => match c {
            None => None,
            Some(c) => c.get(1).map(|s| s.as_str()),
        },
    };
    let desc = match field_effect_value {
        None => desc.to_string(),
        Some(v) => RE_FIELD_EFFECT
            .replace(desc, format!("**[Field Effect]{v}**"))
            .to_string(),
    };

    let splits = desc.split("\n").collect::<Vec<&str>>();
    let index = splits.iter().position(|line| line.starts_with("[1]"));

    let desc_without_footnotes = match index {
        None => splits.join("\n"),
        Some(index) => splits[..index].join("\n"),
    };

    let desc_without_citations = RE_CITATION.replace_all(&desc_without_footnotes, "");
    let replaced_emojis = wiki_symbols_to_emojis(&desc_without_citations);
    replaced_emojis
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
        Some(d) => format_description_markdown(d),
        None => String::from("N/A"),
    };

    (title, description, false)
}

fn format_as(
    tag: &str,
    english: &str,
    japanese: &Option<String>,
    level: &Option<String>,
    special_training: bool,
    color: &Option<String>,
    activation_count: &Option<String>,
    description: &Option<String>,
) -> (String, String, bool) {
    let english = match special_training {
        true => format!("{english} (+)"),
        false => match level {
            Some(lv) => format!("{} Lv. {}", english, lv),
            None => String::from(english),
        },
    };

    let japanese = match japanese {
        None => "".to_string(),
        Some(japanese) => match special_training {
            true => format!("{japanese}(+)"),
            false => match level {
                Some(lv) => format!("({} Lv. {})", japanese, lv),
                None => format!("({})", japanese),
            },
        },
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
            "h" => "<:heartbox:792798475510612009>",
            _ => "?",
        },
        None => "?",
    };

    let title = trim_whitespace(&format!(
        "[{tag}] {english} {japanese} [{activation_puyo_emoji}×{activation_count}]"
    ));
    let description = match description {
        Some(d) => format_description_markdown(d),
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

    if let (None, Some(_lste)) = (&wiki_template.lst, &wiki_template.lste) {
        fields.push(format_ls(
            "LS+",
            &format!("{} SP", wiki_template.name),
            &(match &wiki_template.jpname {
                Some(jpname) => Some(format!("{} SP", jpname)),
                None => None,
            }),
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
            false,
            &wiki_template.color,
            &wiki_template.asn,
            &wiki_template.ase,
        ));
    }

    if let (Some(active_skill), Some(asfe)) = (&wiki_template.r#as, &wiki_template.asfe) {
        let tag = match &card.wiki_template.dsn {
            None => "AS:FP",
            Some(_) => "AS:DS",
        };

        fields.push(format_as(
            tag,
            &active_skill,
            &wiki_template.jpas,
            &wiki_template.aslv,
            false,
            &wiki_template.color,
            &wiki_template.asfn,
            &Some(asfe.clone()),
        ));
    }

    if let Some(ast) = &wiki_template.ast {
        fields.push(format_as(
            "AS+",
            &ast,
            &wiki_template.jpast,
            &None,
            false,
            &wiki_template.color,
            &wiki_template.astn,
            &wiki_template.aste,
        ));
    }

    match (&wiki_template.r#as, &wiki_template.ast, &wiki_template.aste) {
        (_, Some(ast), Some(_aste)) => {
            fields.push(format_as(
                "AS+",
                &ast,
                &wiki_template.jpast,
                &None,
                false,
                &wiki_template.color,
                &wiki_template.astn,
                &wiki_template.aste,
            ));
        }
        (Some(active_skill), None, Some(_aste)) => {
            fields.push(format_as(
                "AS+",
                &active_skill,
                &wiki_template.jpas,
                &None,
                true,
                &wiki_template.color,
                &wiki_template.astn,
                &wiki_template.aste,
            ));
        }
        _ => {}
    }

    if let Some(ast2) = &wiki_template.ast2 {
        fields.push(format_as(
            "AS+",
            &ast2,
            &wiki_template.jpast2,
            &None,
            false,
            &wiki_template.color,
            &wiki_template.ast2n,
            &wiki_template.ast2e,
        ));
    }

    if let Some(ast3) = &wiki_template.ast3 {
        fields.push(format_as(
            "AS+",
            &ast3,
            &wiki_template.jpast3,
            &None,
            false,
            &wiki_template.color,
            &wiki_template.ast3n,
            &wiki_template.ast3e,
        ));
    }

    let has_high_bslv = match &wiki_template.bslv {
        None => false,
        Some(lv) => match lv.parse::<i32>() {
            Ok(lv) => lv >= 15,
            Err(_) => false,
        },
    };
    // Only show battle skills if it's >=Lv15, or if the card only has a BS (and no AS)
    if let (Some(bs), true, _) = (&wiki_template.bs, has_high_bslv, &wiki_template.r#as) {
        fields.push(format_as(
            "BS",
            &bs,
            &wiki_template.jpbs,
            &wiki_template.bslv,
            false,
            &wiki_template.color,
            &wiki_template.bsn,
            &wiki_template.bse,
        ));
    } else if let (Some(bs), _, None) = (&wiki_template.bs, has_high_bslv, &wiki_template.r#as) {
        fields.push(format_as(
            "BS",
            &bs,
            &wiki_template.jpbs,
            &wiki_template.bslv,
            false,
            &wiki_template.color,
            &wiki_template.bsn,
            &wiki_template.bse,
        ));
    }

    if let Some(ca) = &wiki_template.ca {
        let cross_ability_activation_color = match (&wiki_template.color, &wiki_template.caalt) {
            (_, Some(caalt)) => Some(caalt.clone()),
            (Some(color), _) => Some(color.clone()),
            (_, _) => None,
        };

        fields.push(format_as(
            "CA",
            &ca,
            &wiki_template.jpca,
            &wiki_template.calv,
            false,
            &cross_ability_activation_color,
            &wiki_template.bsn,
            &wiki_template.bse,
        ));
    }

    fields
}

pub fn format_card_title(card: &Card) -> String {
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

    let description = match &card.wiki_template.dsn {
        None => format!(
            r#"HP: {hp}　ATK: {atk}　RCV: {rcv}
Cost: {cost}　Type: {card_type}"#
        ),
        Some(dsn) => format!(
            r#"HP: {hp}　ATK: {atk}　RCV: {rcv}
Cost: {cost}　Type: {card_type}
Dual Shift: <:dualshift:1314042310291623937>{dsn}"#
        ),
    };

    (title, description)
}

pub fn format_combination_categories(card: &Card) -> Option<String> {
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
        Some(s) => {
            // Some cards (e.g. material cards) have dummy combin1 values
            if s == "--" {
                None
            } else {
                Some(markdown::combination_link(s))
            }
        }
        None => None,
    })
    .collect::<Vec<String>>()
    .join(" ");

    if combins.len() > 0 {
        Some(format!("<:combination:792798801034346558> {combins}"))
    } else {
        None
    }
}

// fn icon_select_row(
//     card_id: &str,
//     card_icon_urls: &CardIconUrls,
// ) -> Option<serenity::CreateActionRow> {
//     let mut buttons: Vec<serenity::CreateButton> = Vec::new();

//     if card_icon_urls.normal.is_some() {
//         buttons.push(
//             serenity::CreateButton::new(card_embed_update_custom_id_builder(
//                 card_id,
//                 CardIconType::Normal,
//             ))
//             .label("REG"),
//         );
//     }
//     if card_icon_urls.dual_shift.is_some() {
//         buttons.push(
//             serenity::CreateButton::new(card_embed_update_custom_id_builder(
//                 card_id,
//                 CardIconType::DualShift,
//             ))
//             .label("DS"),
//         );
//     }
//     if card_icon_urls.extra_power.is_some() {
//         buttons.push(
//             serenity::CreateButton::new(card_embed_update_custom_id_builder(
//                 card_id,
//                 CardIconType::ExtraPower,
//             ))
//             .label("EP"),
//         );
//     }
//     if card_icon_urls.extra_power_dual_shift.is_some() {
//         buttons.push(
//             serenity::CreateButton::new(card_embed_update_custom_id_builder(
//                 card_id,
//                 CardIconType::ExtraPowerDualShift,
//             ))
//             .label("EP+DS"),
//         );
//     }

//     if buttons.len() > 1 {
//         Some(serenity::CreateActionRow::Buttons(buttons))
//     } else {
//         None
//     }
// }

fn embed_navigation(card: &Card) -> serenity::CreateActionRow {
    let buttons: Vec<serenity::CreateButton> = vec![
        character_nav_button(&card.char_id),
        full_art_nav_button(&card.card_id),
        lore_nav_button(&card.card_id),
    ];

    serenity::CreateActionRow::Buttons(buttons)
}
