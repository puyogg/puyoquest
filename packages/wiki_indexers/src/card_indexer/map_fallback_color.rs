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
