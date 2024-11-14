use fancy_regex::Regex;

lazy_static::lazy_static! {
    static ref RE_RARITY_SUFFIX: Regex = Regex::new(r"\/★.*$").unwrap();
}

pub fn format_card_link_name(
    link_name: &str,
    rarity: &str,
    rarity_modifier: &Option<String>,
) -> String {
    let suffix_match = RE_RARITY_SUFFIX.is_match(link_name);
    let suffix_match = match suffix_match {
        Err(_) => false,
        Ok(s) => s,
    };

    if suffix_match {
        return String::from(link_name);
    }

    match rarity_modifier.as_deref() {
        None => {
            format!("{link_name}/★{rarity}")
        }
        Some(rm) => {
            format!("{link_name}/★{rm}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_through_link_name_that_already_has_rarity() {
        let result = format_card_link_name("Santa Ringo/★6-2", "6", &Some("6-2".to_string()));
        assert_eq!(result, "Santa Ringo/★6-2");
    }

    #[test]
    fn adds_rarity_to_normal_link_name() {
        let result = format_card_link_name("Arle", "7", &None);
        assert_eq!(result, "Arle/★7");
    }

    #[test]
    fn adds_rarity_modifier_if_link_name_didnt_have_rarity_already_somehow() {
        let result = format_card_link_name("Santa Ringo", "6", &Some("6-2".to_string()));
        assert_eq!(result, "Santa Ringo/★6-2");
    }
}
