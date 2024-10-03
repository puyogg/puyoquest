#[derive(Debug, PartialEq, Eq)]
pub struct RarityAndModifier {
    pub rarity: String,
    pub modifier: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RarityParsingError {
    received: String,
}

impl std::fmt::Display for RarityParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid rarity: {}", self.received)
    }
}

const VALID_RARITIES: &'static [&str] = &["1", "2", "3", "4", "5", "6", "6s", "6-1", "6-2", "7"]; 

pub fn parse_rarity(text: &str) -> Result<RarityAndModifier, RarityParsingError> {
    let text = text.to_lowercase();

    let is_valid_rarity_query = VALID_RARITIES.contains(&text.as_ref());
    if !is_valid_rarity_query {
        return Err(RarityParsingError {
            received: text
        })
    };

    let rarity_and_modifier = match text.as_str() {
        "6s" => RarityAndModifier {
            rarity: "6".to_string(),
            modifier: Some("6-2".to_string()),
        },
        "6-2" => RarityAndModifier {
            rarity: "6".to_string(),
            modifier: Some("6-2".to_string()),
        },
        "6-1" => RarityAndModifier {
            rarity: "6".to_string(),
            modifier: Some("6-1".to_string()),
        },
        _ => RarityAndModifier {
            rarity: text,
            modifier: None,
        },
    };

    Ok(rarity_and_modifier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_normal_rarity() {
        let result = parse_rarity("6").unwrap();
        assert_eq!(result, RarityAndModifier { rarity: "6".to_string(), modifier: None });
    }

    #[test]
    fn parses_6s_rarity() {
        let result = parse_rarity("6S").unwrap();
        assert_eq!(result, RarityAndModifier { rarity: "6".to_string(), modifier: Some("6-2".to_string()) });
    }

    #[test]
    fn parses_6_2_rarity() {
        let result = parse_rarity("6-2").unwrap();
        assert_eq!(result, RarityAndModifier { rarity: "6".to_string(), modifier: Some("6-2".to_string()) });
    }

    #[test]
    fn parses_6_1_rarity() {
        let result = parse_rarity("6-1").unwrap();
        assert_eq!(result, RarityAndModifier { rarity: "6".to_string(), modifier: Some("6-1".to_string()) });
    }

    #[test]
    fn invalid_normal_rarity() {
        let result = parse_rarity("24").unwrap_err();
        assert_eq!(result.received, "24".to_string());
    }

    #[test]
    fn invalid_special_rarity() {
        let result = parse_rarity("6-4").unwrap_err();
        assert_eq!(result.received, "6-4".to_string());
    }
}
