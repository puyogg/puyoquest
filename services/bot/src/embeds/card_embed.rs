use poise::serenity_prelude as serenity;

pub fn card_embed() -> serenity::CreateEmbed {
    let image_url = "https://raw.githubusercontent.com/serenity-rs/serenity/current/logo.png";
    let embed = serenity::CreateEmbed::default()
        .description("embed 1")
        .image(image_url);

    embed
}