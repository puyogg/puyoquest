use crate::commands::{Data, Error};
use poise::serenity_prelude as serenity;

pub mod button;
pub mod string_select;
mod permissions;
pub use permissions::{check_if_wiki_editor, check_if_wiki_editor_and_error};

pub async fn component_router(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
) -> Result<(), Error> {
    match &interaction.data.kind {
        serenity::ComponentInteractionDataKind::Button => {
            button::button_router(ctx, data, interaction).await?;
        },
        serenity::ComponentInteractionDataKind::StringSelect { values } => {
            string_select::string_select_router(ctx, data, interaction, values).await?;
        }
        _ => {}
    };
    
    Ok(())
}