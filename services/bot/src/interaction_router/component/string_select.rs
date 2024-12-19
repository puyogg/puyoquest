use crate::commands::{Data, Error};
use poise::serenity_prelude as serenity;

use super::permissions::check_from_original_user;

mod card_handler;
mod did_you_mean_handler;
mod full_art_handler;

pub async fn string_select_router(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
    values: &Vec<String>,
) -> Result<(), Error> {
    let custom_id = &interaction.data.custom_id;

    if custom_id.starts_with("card:") {
        check_from_original_user(ctx, interaction).await?;
        card_handler::card_handler(ctx, data, interaction, values).await?;
    }

    if custom_id.starts_with("full_art:") {
        check_from_original_user(ctx, interaction).await?;
        full_art_handler::full_art_handler(ctx, data, interaction, values).await?;
    }

    if custom_id.starts_with("did_you_mean:") {
        check_from_original_user(ctx, interaction).await?;
        did_you_mean_handler::did_you_mean_handler(ctx, data, interaction, values).await?;
    }

    Ok(())
}
