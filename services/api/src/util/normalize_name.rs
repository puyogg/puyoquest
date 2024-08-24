use fancy_regex::Regex;
use unicode_normalization::UnicodeNormalization;

lazy_static::lazy_static! {
    static ref RE_STAR: Regex = Regex::new(r"☆").unwrap();

    // e.g. Santa Ringo/★6S
    static ref RE_RARITY_SUFFIX: Regex = Regex::new(r"\/★.*").unwrap();

    static ref RE_SPECIAL_CHARACTERS: Regex = Regex::new(r"[\u0300-\u036f]").unwrap();
    static ref RE_MULTIWHITESPACE: Regex = Regex::new(r"\s\s+").unwrap();
    static ref RE_WHITESPACE: Regex = Regex::new(r"\s").unwrap();

    static ref RE_S_MODIFIER_ENG: Regex = Regex::new(r"\sS$").unwrap();
    static ref RE_S_MODIFIER_JP: Regex = Regex::new(r"・S$").unwrap();
}

pub fn normalize_name(text: &str) -> String {
    let trimmed = text.trim();
    let no_star = RE_STAR.replace_all(trimmed, " ");
    let no_rarity = RE_RARITY_SUFFIX.replace(&no_star, "");

    let remove_s_modifier_eng = RE_S_MODIFIER_ENG.replace(&no_rarity, "");
    let remove_s_modifier_jp = RE_S_MODIFIER_JP.replace(&remove_s_modifier_eng, "");

    let nfkd = remove_s_modifier_jp.nfkd().to_string();

    let replaced_special = RE_SPECIAL_CHARACTERS.replace_all(&nfkd, "");
    let replaced_multiwhitespace = RE_MULTIWHITESPACE.replace_all(&replaced_special, " ");
    let replaced_whitespace = RE_WHITESPACE.replace_all(&replaced_multiwhitespace, " ");

    let result = replaced_whitespace.to_lowercase();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercases_latin_text() {
        let result = normalize_name("MEIKO");
        assert_eq!(result, "meiko");
    }

    #[test]
    fn removes_diacritics() {
        let result = normalize_name("Saucy Legamünt");
        assert_eq!(result, "saucy legamunt");
    }

    #[test]
    fn replaces_star_with_a_space() {
        let result = normalize_name("Space☆Ecolo");
        assert_eq!(result, "space ecolo");
    }

    #[test]
    fn removes_slash_rarity_star() {
        let result = normalize_name("Paprisu/Red/★4");
        assert_eq!(result, "paprisu/red");
    }

    #[test]
    fn converts_full_width_spaces_to_half_width() {
        let result = normalize_name("Puyo Puyo　Tetris 2");
        assert_eq!(result, "puyo puyo tetris 2");
    }

    #[test]
    fn trims_and_collapses_whitespace() {
        let result = normalize_name(" 　 Space    Ecolo   ");
        assert_eq!(result, "space ecolo");
    }

    #[test]
    fn removes_s_modifier() {
        let result = normalize_name("Santa Ringo S");
        assert_eq!(result, "santa ringo");
    }

    #[test]
    fn removes_s_modifier_jp_names() {
        let result = normalize_name("サンタりんご・S");
        assert_eq!(result, "サンタりんご");
    }
}
