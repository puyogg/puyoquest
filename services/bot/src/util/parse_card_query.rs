use fancy_regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub struct AliasAndRarity {
    pub alias: String,
    pub rarity: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct AliasAndRarityQuery {
    pub query: Option<AliasAndRarity>,
    pub fallback: String,
}

lazy_static::lazy_static! {
    static ref RE_MULTIWHITESPACE: Regex = Regex::new(r"\s\s+").unwrap();
    static ref RE_ALIAS_AND_RARITY_GROUPS: Regex = Regex::new(r"(.+)( )(\d.*$)").unwrap();
}

pub fn parse_alias_and_rarity(text: impl AsRef<str>) -> AliasAndRarityQuery {
    let text = text.as_ref();
    let text = text.trim();
    let trimmed = RE_MULTIWHITESPACE.replace_all(text, " ");

    let captures = RE_ALIAS_AND_RARITY_GROUPS.captures(&trimmed).ok().flatten();

    match captures {
        Some(c) => {
            let alias = c.get(1);
            let rarity = c.get(3);

            match (alias, rarity) {
                (Some(a), Some(r)) => AliasAndRarityQuery {
                    query: Some(AliasAndRarity {
                        alias: a.as_str().to_string(),
                        rarity: r.as_str().to_string(),
                    }),
                    fallback: trimmed.to_string(),
                },
                _ => AliasAndRarityQuery {
                    query: None,
                    fallback: trimmed.to_string(),
                },
            }
        }
        None => AliasAndRarityQuery {
            query: None,
            fallback: trimmed.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_arle_7_to_query() {
        let result = parse_alias_and_rarity("  Arle  7 ");

        assert_eq!(
            result,
            AliasAndRarityQuery {
                query: Some(AliasAndRarity {
                    alias: "Arle".to_string(),
                    rarity: "7".to_string(),
                }),
                fallback: "Arle 7".to_string(),
            }
        );
    }

    #[test]
    fn parses_arle_to_fallback_only() {
        let result = parse_alias_and_rarity("  Arle     ");

        assert_eq!(
            result,
            AliasAndRarityQuery {
                query: None,
                fallback: "Arle".to_string(),
            }
        )
    }

    #[test]
    fn parses_santa_ringo_to_query() {
        let result = parse_alias_and_rarity("  Santa Ringo   6-2 ");

        assert_eq!(
            result,
            AliasAndRarityQuery {
                query: Some(AliasAndRarity {
                    alias: "Santa Ringo".to_string(),
                    rarity: "6-2".to_string(),
                }),
                fallback: "Santa Ringo 6-2".to_string(),
            }
        );
    }

    #[test]
    fn parses_santa_ringo_to_fallback_only() {
        let result = parse_alias_and_rarity("  Santa Ringo   ");

        assert_eq!(
            result,
            AliasAndRarityQuery {
                query: None,
                fallback: "Santa Ringo".to_string(),
            }
        )
    }

    #[test]
    fn parses_name_with_trailing_number_to_query() {
        let result = parse_alias_and_rarity("Schezo ver. Division 24 6");

        assert_eq!(
            result,
            AliasAndRarityQuery {
                query: Some(AliasAndRarity {
                    alias: "Schezo ver. Division 24".to_string(),
                    rarity: "6".to_string(),
                }),
                fallback: "Schezo ver. Division 24 6".to_string(),
            }
        );
    }

    #[test]
    fn parses_name_with_trailing_number_to_query_still() {
        let result = parse_alias_and_rarity("Schezo ver. Division 24");

        assert_eq!(
            result,
            AliasAndRarityQuery {
                query: Some(AliasAndRarity {
                    alias: "Schezo ver. Division".to_string(),
                    rarity: "24".to_string(),
                }),
                fallback: "Schezo ver. Division 24".to_string(),
            }
        );
    }
}
