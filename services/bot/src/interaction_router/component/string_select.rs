use poise::serenity_prelude as serenity;
use crate::commands::{Data, Error};

mod card_handler;
mod full_art_handler;

pub async fn string_select_router(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
    values: &Vec<String>,
) -> Result<(), Error> {
    let custom_id = &interaction.data.custom_id;

    println!("{}, {:?}", custom_id, values);

    if custom_id.starts_with("card:") {
        card_handler::card_handler(ctx, data, interaction, values).await?;
    }

    if custom_id.starts_with("full_art:") {
        full_art_handler::full_art_handler(ctx, data, interaction, values).await?;
    }

    Ok(())
}