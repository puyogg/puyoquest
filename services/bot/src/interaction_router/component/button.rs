use poise::serenity_prelude as serenity;
use crate::commands::{Data, Error};

pub mod card_handler;
pub mod character_handler;

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
        card_handler::card_handler(ctx, data, interaction).await?;
    }
    
    if custom_id.starts_with("character:") {
        character_handler::character_handler(ctx, data, interaction).await?;
    }

    Ok(())
}
