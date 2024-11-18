use urlencoding::encode;

pub fn combination_link(name: &str) -> String {
    let url = format!("https://puyonexus.com/wiki/Category:PPQ:{}_Combination", encode(name));

    format!("[[{}]]({})", name, url)
}

pub fn series_link(name: &str) -> String {
    let url = format!("https://puyonexus.com/wiki/Category:PPQ:{}_Series", encode(name));

    format!("[[{}]]({})", name, url)
}
