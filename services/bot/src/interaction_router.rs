use crate::commands::{Data, Error};
use poise::serenity_prelude as serenity;

pub mod component;

pub async fn interaction_router(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::Interaction,
) -> Result<(), Error> {
    match interaction {
        serenity::Interaction::Component(component_interaction) => {
            component::component_router(ctx, data, component_interaction).await?;
        },
        _ => {},
    };

    Ok(())
}
