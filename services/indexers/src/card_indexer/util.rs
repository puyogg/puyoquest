use std::sync::LazyLock;
use fancy_regex::Regex;

pub fn map_fallback_color(c: &str) -> Option<String> {
    let result = match c {
        "1" => Some("Red"),
        "2" => Some("Blue"),
        "3" => Some("Green"),
        "4" => Some("Yellow"),
        "5" => Some("Purple"),
        _ => None,
    };

    result.map(|r| r.to_string())
}

pub static RE_RARITY_MODIFIER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\/(★\d.*)$").unwrap());

pub fn parse_rarity_modifier(link_name: &str) -> Option<String> {
    let captures = RE_RARITY_MODIFIER.captures(link_name);
    let captures = match captures {
        Err(e) => {
            println!("Regex error trying to parse link_name: {}", link_name);
            println!("{e}");
            return None;
        }
        Ok(c) => match c {
            None => return None,
            Some(c) => c,
        },
    };

    let rarity_modifier = captures.get(1);
    let rarity_modifier = match rarity_modifier {
        None => return None,
        Some(r) => r.as_str().to_string(),
    };

    Some(rarity_modifier)
}
