use urlencoding::encode;
use sdk::models::Card;

pub fn combination_link(name: &str) -> String {
    let url = format!("https://puyonexus.com/wiki/Category:PPQ:{}_Combination", encode(name));

    format!("[[{}]]({})", name, url)
}

pub fn series_link(name: &str) -> String {
    let url = format!("https://puyonexus.com/wiki/Category:PPQ:{}_Series", encode(name));

    format!("[[{}]]({})", name, url)
}

pub fn rarity_link_list(cards: &Vec<Card>) -> String {
    let markdown_links: Vec<String> = cards.into_iter().map(|c| {
        let rarity = match &c.rarity_modifier {
            Some(rm) => rm,
            None => &c.rarity,
        };
        let url = &c.url;

        format!("[[★{rarity}]]({url})")
    }).collect();

    let markdown = markdown_links.join(" ");
    markdown
}

pub fn material_link_list(cards: &Vec<Card>) -> String {
    let markdown_links: Vec<String> = cards.into_iter().map(|c| {
        let rarity = match &c.rarity_modifier {
            Some(rm) => rm,
            None => &c.rarity,
        };
        let url = &c.url;
        let name = &c.name;

        format!("[[{name} ★{rarity}]]({url})")
    }).collect();

    let markdown = markdown_links.join(" ");
    markdown
}
