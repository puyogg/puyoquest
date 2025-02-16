use fancy_regex::Regex;
use std::sync::LazyLock;

pub static RE_RARITY_SUFFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"/.*?$").unwrap());

pub fn remove_rarity_suffix(link_name: &str) -> String {
    RE_RARITY_SUFFIX.replace(link_name, "").as_ref().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_slash_and_rarity() {
        let link_name = "Santa Ringo/★6-2";
        let replaced = remove_rarity_suffix(link_name);
        assert_eq!(replaced, "Santa Ringo");
    }
}
