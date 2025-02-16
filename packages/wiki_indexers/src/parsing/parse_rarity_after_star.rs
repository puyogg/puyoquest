use fancy_regex::Regex;
use std::sync::LazyLock;

use super::error::ParsingError;

pub static RE_RARITY_AFTER_STAR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"/★(\d.*)$").unwrap());

pub fn parse_rarity_after_star(link_name: &str) -> Result<Option<String>, ParsingError> {
    let captures = RE_RARITY_AFTER_STAR.captures(link_name)?;

    let captures = match captures {
        Some(c) => c,
        None => return Ok(None),
    };

    let rarity = captures.get(1);
    let rarity = match rarity {
        None => return Ok(None),
        Some(r) => r.as_str().to_string(),
    };

    Ok(Some(rarity))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rarity() {
        let link_name = "Arle/★7";
        let rarity = parse_rarity_after_star(link_name).unwrap().unwrap();
        assert_eq!(rarity, "7");
    }

    #[test]
    fn parses_6_2() {
        let link_name = "Santa Ringo/★6-2";
        let rarity = parse_rarity_after_star(link_name).unwrap().unwrap();
        assert_eq!(rarity, "6-2");
    }

    #[test]
    fn none() {
        let link_name = "Santa Ringo";
        let rarity = parse_rarity_after_star(link_name).unwrap();
        assert_eq!(rarity, None);
    }
}
