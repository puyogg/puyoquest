use crate::commands::{Data, Error};
use crate::embeds;
use crate::embeds::CardIconType;
use poise::serenity_prelude as serenity;

pub async fn card_handler(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
    values: &Vec<String>,
) -> Result<(), Error> {
    let card_id = values.get(0);
    let card_id = match card_id {
        None => {
            let response = serenity::CreateInteractionResponseMessage::default()
                .content("Failed to receive card id! Tell S2 about this error.")
                .ephemeral(true);

            interaction
                .create_response(ctx, serenity::CreateInteractionResponse::Message(response))
                .await?;
            return Ok(());
        }
        Some(c) => c,
    };

    let card = sdk::apis::cards_api::cards_id_get(&data.api_config, &card_id).await?;

    let (card_embed, components) = embeds::card_embed(&card, CardIconType::Normal);
    let response = serenity::CreateInteractionResponseMessage::default().embed(card_embed);
    let response = if components.len() > 0 {
        response.components(components)
    } else {
        response
    };

    // Assume interaction update response_type for now.
    interaction
        .create_response(
            ctx,
            serenity::CreateInteractionResponse::UpdateMessage(response),
        )
        .await?;

    Ok(())
}
