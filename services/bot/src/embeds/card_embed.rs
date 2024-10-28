use poise::serenity_prelude as serenity;
use sdk::models::Card;

use crate::util::emoji_table::wiki_symbols_to_emojis;
use crate::util::embed_colors;

pub fn card_embed(card: &Card) -> serenity::CreateEmbed {
    let embed = serenity::CreateEmbed::default()
        .description("embed 1")
        .fields(first_page(card));

    let embed = match card.main_color.as_str() {
        "Red" => embed.color(embed_colors::RED),
        "Blue" => embed.color(embed_colors::BLUE),
        "Green" => embed.color(embed_colors::GREEN),
        "Yellow" => embed.color(embed_colors::YELLOW),
        "Purple" => embed.color(embed_colors::PURPLE),
        _ => embed,
    };
    
    embed
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

fn first_page(card: &Card) -> Vec<(String, String, bool)> {
    let wiki_template = &card.wiki_template;

    let mut fields: Vec<(String, String, bool)> = Vec::new();

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

    fields
}
