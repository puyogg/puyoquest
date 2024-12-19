use crate::commands::{Data, Error};
use crate::embeds;
use poise::serenity_prelude as serenity;

pub async fn did_you_mean_handler(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
    values: &Vec<String>,
) -> Result<(), Error> {
    let char_id = values.get(0);
    let char_id = match char_id {
        None => {
            let response = serenity::CreateInteractionResponseMessage::default()
                .content("Failed to receive char_id! Tell S2 about this error.")
                .ephemeral(true);

            interaction
                .create_response(ctx, serenity::CreateInteractionResponse::Message(response))
                .await?;
            return Ok(());
        }
        Some(c) => c,
    };

    let character =
        sdk::apis::characters_api::characters_id_get(&data.api_config, &char_id).await?;
    let cards_and_materials = sdk::apis::characters_api::characters_id_cards_get(
        &data.api_config,
        &char_id,
        Some("false"),
    )
    .await?;

    let (embed, components) = embeds::character_embed(&character, &cards_and_materials);
    let response = serenity::CreateInteractionResponseMessage::default().embed(embed);
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
