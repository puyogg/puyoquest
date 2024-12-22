use crate::commands::{Data, Error};
use poise::serenity_prelude as serenity;

use super::permissions::check_from_original_user;

pub mod art_handler;
pub mod card_handler;
pub mod character_handler;
pub mod full_art_handler;
pub mod lore_handler;
pub mod alias_handler;

/// Button responses have this format in the interaction's custom_id:
///
/// {target_embed_type}:{response_type}:{TBD}:{api_id}:{...}
pub async fn button_router(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
) -> Result<(), Error> {
    let custom_id = &interaction.data.custom_id;

    if custom_id.starts_with("card:") {
        check_from_original_user(ctx, interaction).await?;
        card_handler::card_handler(ctx, data, interaction).await?;
    }

    if custom_id.starts_with("character:") {
        check_from_original_user(ctx, interaction).await?;
        character_handler::character_handler(ctx, data, interaction).await?;
    }

    if custom_id.starts_with("lore:") {
        check_from_original_user(ctx, interaction).await?;
        lore_handler::lore_handler(ctx, data, interaction).await?;
    }

    if custom_id.starts_with("full_art:") {
        check_from_original_user(ctx, interaction).await?;
        full_art_handler::full_art_handler(ctx, data, interaction).await?;
    }

    if custom_id.starts_with("alias:") {
        check_from_original_user(ctx, interaction).await?;
        alias_handler::alias_handler(ctx, data, interaction).await?;
    }

    Ok(())
}
