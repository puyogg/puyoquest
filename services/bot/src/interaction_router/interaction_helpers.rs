use crate::commands::Error;
use poise::serenity_prelude::{self as serenity, ComponentInteraction, Error as SerenityError};

pub async fn basic_component_response(
    ctx: &serenity::Context,
    interaction: &ComponentInteraction,
    message: impl AsRef<str>,
) -> Result<(), Error> {
    let message = message.as_ref();
    interaction
        .create_response(
            ctx,
            serenity::CreateInteractionResponse::Message(
                serenity::CreateInteractionResponseMessage::new().content(message),
            ),
        )
        .await?;

    Ok(())
}
