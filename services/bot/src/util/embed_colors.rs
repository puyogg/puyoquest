use poise::serenity_prelude as serenity;
use serenity::model::Color;

pub const RED: Color = Color::from_rgb(223, 17, 17);
pub const BLUE: Color = Color::from_rgb(19, 70, 223);
pub const GREEN: Color = Color::from_rgb(16, 155, 8);
pub const YELLOW: Color = Color::from_rgb(250, 157, 14);
pub const PURPLE: Color = Color::from_rgb(153, 26, 217);

pub fn color_from_str(s: &str) -> Option<Color> {
    match s.to_lowercase().as_str() {
        "red" => Some(RED),
        "blue" => Some(BLUE),
        "green" => Some(GREEN),
        "yellow" => Some(YELLOW),
        "purple" => Some(PURPLE),
        _ => return None,
    }
}
