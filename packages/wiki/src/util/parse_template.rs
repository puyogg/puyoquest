use fancy_regex::{Captures, Regex};
use serde_json::Map;

lazy_static::lazy_static! {
    /// Matches HTML-style comments in mediawiki templates
    static ref RE_MEDIAWIKI_COMMENT: Regex = Regex::new(r"<!--[\S\s]+-->").unwrap();
    /// Mediawiki template keys have the format: |name1=
    /// This regex matches that and splits it into 3 capture groups: |, {name}, =
    static ref RE_TEMPLATE_KEY: Regex = Regex::new(r"(\|)(\w+?)(=)").unwrap();
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct BracketPair {
    start: usize,
    end: usize,
    depth: i32,
}

#[derive(Debug, PartialEq)]
pub enum FindBracketPairsError {
    UnbalancedBrackets,
}

impl std::error::Error for FindBracketPairsError {}
impl std::fmt::Display for FindBracketPairsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnbalancedBrackets => write!(f, "Failed to parse template: Unbalanced brackets"),
        }
    }
}

fn find_bracket_pairs(raw_template: &str) -> Result<Vec<BracketPair>, FindBracketPairsError> {
    let template_no_comments = RE_MEDIAWIKI_COMMENT.replace_all(raw_template, "");

    let opening_indices = template_no_comments.match_indices("{");
    let closing_indices = template_no_comments.match_indices("}");
    if opening_indices.count() != closing_indices.count() {
        return Err(FindBracketPairsError::UnbalancedBrackets);
    }

    let brackets: Vec<(usize, &str)> = template_no_comments
        .match_indices(|x| x == '{' || x == '}')
        .collect();

    let mut depth = 0;
    let mut bracket_pairs: Vec<BracketPair> = Vec::new();
    let mut stack: Vec<(usize, &str)> = Vec::new();

    for (index, token) in brackets.iter() {
        let index = *index;
        let token = *token;

        if token == "{" {
            stack.push((index, token));
            depth += 1;
        } else if token == "}" {
            let (start, _) = stack
                .pop()
                .ok_or(FindBracketPairsError::UnbalancedBrackets)?;
            depth -= 1;
            bracket_pairs.push(BracketPair {
                start,
                end: index,
                depth,
            })
        }
    }

    Ok(bracket_pairs)
}

#[derive(Debug)]
pub enum ParseTemplateError {
    FindBracketPairs(FindBracketPairsError),
    TemplateKey,
    /// Key, Raw Template
    RegexError(String),
    KeyMatchingError,
    JsonDeserialization(serde_json::Error),
}

impl std::error::Error for ParseTemplateError {}
impl std::fmt::Display for ParseTemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseTemplateError::FindBracketPairs(bracket_error) => bracket_error.fmt(f),
            ParseTemplateError::TemplateKey => write!(f, "Failed to parse template: Key Error"),
            ParseTemplateError::JsonDeserialization(json_error) => json_error.fmt(f),
            ParseTemplateError::RegexError(s) => write!(f, "{}", s),
            ParseTemplateError::KeyMatchingError => {
                write!(f, "Don't really know why this happened but it did!")
            }
        }
    }
}

impl From<FindBracketPairsError> for ParseTemplateError {
    fn from(e: FindBracketPairsError) -> Self {
        Self::FindBracketPairs(e)
    }
}

impl From<serde_json::Error> for ParseTemplateError {
    fn from(e: serde_json::Error) -> Self {
        Self::JsonDeserialization(e)
    }
}

pub fn parse_template(raw_template: &str) -> Result<serde_json::Value, ParseTemplateError> {
    let bracket_pairs = find_bracket_pairs(raw_template)?;
    let template = RE_MEDIAWIKI_COMMENT.replace_all(raw_template, "");

    let deep_pairs = bracket_pairs
        .iter()
        .filter(|pair| pair.depth >= 2)
        .collect::<Vec<&BracketPair>>();

    let capture_group_sets = RE_TEMPLATE_KEY
        .captures_iter(&template)
        .filter_map(|capture| capture.ok())
        .collect::<Vec<Captures>>();

    let capture_group_sets = capture_group_sets
        .iter()
        .filter_map(|capture_groups| {
            let key = capture_groups.get(2)?;
            let start = key.start();

            let key_in_deep_pairs = deep_pairs
                .iter()
                .find(|pair| pair.start < start && start < pair.end).is_some();

            match key_in_deep_pairs {
                true => None,
                false => Some(capture_groups),
            }
        })
        .collect::<Vec<&Captures>>();

    let mut key_value_map: Map<String, serde_json::Value> = Map::new();
    for (pos, capture_groups) in capture_group_sets.iter().enumerate() {
        let key = capture_groups
            .get(2)
            .ok_or(ParseTemplateError::RegexError(raw_template.to_string()))?;

        let value_start = capture_groups
            .get(3)
            .ok_or(ParseTemplateError::RegexError(raw_template.to_string()))?
            .end();

        let next_group = if pos < capture_group_sets.len() - 1 {
            capture_group_sets.get(pos + 1)
        } else {
            None
        };

        let value_end_bound = match next_group {
            Some(next_group) => {
                next_group
                    .get(0)
                    .ok_or(ParseTemplateError::RegexError(raw_template.to_string()))?
                    .start()
            },
            None => template.len() - 2 // Ignore closing }}
        };

        let value = template[value_start..value_end_bound].trim();
        let key = key.as_str();
        key_value_map.insert(
            String::from(key),
            serde_json::Value::String(value.to_string()),
        );
    }

    Ok(serde_json::Value::Object(key_value_map))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn finds_bracket_pairs() {
        let result = find_bracket_pairs(r#"{{{a}}}"#).unwrap();

        let expected_pairs = vec![
            BracketPair {
                start: 2,
                end: 4,
                depth: 2,
            },
            BracketPair {
                start: 1,
                end: 5,
                depth: 1,
            },
            BracketPair {
                start: 0,
                end: 6,
                depth: 0,
            },
        ];

        assert_eq!(result.len(), expected_pairs.len());

        for (i, pair) in result.iter().enumerate() {
            let expected_pair = expected_pairs.get(i).unwrap();
            assert_eq!(pair, expected_pair);
        }
    }

    #[test]
    fn throws_unbalanced_brackets() {
        let result = find_bracket_pairs(r#"{}}"#);
        assert!(result.is_err_and(|e| e == FindBracketPairsError::UnbalancedBrackets));
    }

    #[test]
    fn empty_vec_for_no_brackets() {
        let result = find_bracket_pairs("").unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn parses_template_to_json() {
        let template = r#"{{Char info/{{{1|line}}}|size={{{size}}}|main=Arle Nadja
|code=2012
|name=Arle
|jpname=アルル
|color=Blue
|type1=Attack
|type2=Single
|voicetrans=V2012

|card1=201203
|card2=201204
|card3=201205
|card4=201206
|card5=201207

|acqg=Magic Stone Gacha / Silver Ticket Gacha / Gold Ticket Gacha / Premium Ticket Gacha / [[PPQ:Once-per-day Free Gacha|Once-per-day Free Gacha]]

|manzai=3
|tokkun=Tmainatkas7ls7
}}"#;
        let result = parse_template(template).unwrap();
        let result_json_str: String = result.to_string();

        let expected_json = json!({
            "acqg": "Magic Stone Gacha / Silver Ticket Gacha / Gold Ticket Gacha / Premium Ticket Gacha / [[PPQ:Once-per-day Free Gacha|Once-per-day Free Gacha]]",
            "card1": "201203",
            "card2": "201204",
            "card3": "201205",
            "card4": "201206",
            "card5": "201207",
            "code": "2012",
            "color": "Blue",
            "jpname": "アルル",
            "main": "Arle Nadja",
            "manzai": "3",
            "name": "Arle",
            "size": "{{{size}}}",
            "tokkun": "Tmainatkas7ls7",
            "type1": "Attack",
            "type2": "Single",
            "voicetrans": "V2012"
        });
        let expected_json_str = expected_json.to_string();

        assert_eq!(result_json_str, expected_json_str);
    }

    #[test]
    fn parses_template_with_comment() {
        let template = r#"{{Char info/{{{1|line}}}|size={{{size}}}|main=Arle Nadja
|code=2012
|name=Arle
|jpname=アルル
|color=Blue
|type1=Attack
|type2=Single
|voicetrans=V2012

<!-- comment -->

|card1=201203
|card2=201204
|card3=201205
|card4=201206
|card5=201207

|acqg=Magic Stone Gacha / Silver Ticket Gacha / Gold Ticket Gacha / Premium Ticket Gacha / [[PPQ:Once-per-day Free Gacha|Once-per-day Free Gacha]]

|manzai=3
|tokkun=Tmainatkas7ls7
}}"#;
        let result = parse_template(template).unwrap();
        let result_json_str: String = result.to_string();

        let expected_json = json!({
            "acqg": "Magic Stone Gacha / Silver Ticket Gacha / Gold Ticket Gacha / Premium Ticket Gacha / [[PPQ:Once-per-day Free Gacha|Once-per-day Free Gacha]]",
            "card1": "201203",
            "card2": "201204",
            "card3": "201205",
            "card4": "201206",
            "card5": "201207",
            "code": "2012",
            "color": "Blue",
            "jpname": "アルル",
            "main": "Arle Nadja",
            "manzai": "3",
            "name": "Arle",
            "size": "{{{size}}}",
            "tokkun": "Tmainatkas7ls7",
            "type1": "Attack",
            "type2": "Single",
            "voicetrans": "V2012"
        });
        let expected_json_str = expected_json.to_string();

        assert_eq!(result_json_str, expected_json_str);
    }
}
