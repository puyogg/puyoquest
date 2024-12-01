use crate::commands::{Data, Error};
use crate::embeds::{self, FullArtType};
use crate::interaction_router::component::button::full_art_handler::parse_full_art_embed_custom_id;
use poise::serenity_prelude as serenity;

pub async fn full_art_handler(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
    values: &Vec<String>,
) -> Result<(), Error> {
    let custom_id = &interaction.data.custom_id;
    let card_id = parse_full_art_embed_custom_id(custom_id)?;
    let card_id = match card_id {
        None => {
            interaction
                .create_response(
                    ctx,
                    serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::new()
                            .content("There was an error handling your full art request!")
                            .ephemeral(true),
                    ),
                )
                .await?;

            return Ok(());
        }
        Some(c) => c,
    };

    let variant = values.get(0);
    let variant = match variant {
        None => {
            interaction
                .create_response(
                    ctx,
                    serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::new()
                            .content("There was an error handling your full art variant!"),
                    ),
                )
                .await?;

            return Ok(());
        }
        Some(v) => FullArtType::from(v.as_str()),
    };

    let card = sdk::apis::cards_api::cards_id_get(&data.api_config, &card_id).await?;
    let full_art =
        sdk::apis::cards_api::cards_card_id_full_art_get(&data.api_config, &card_id).await?;

    let (embed, components) = embeds::full_art_embed(&card, &full_art, variant);
    let response = serenity::CreateInteractionResponseMessage::default().embed(embed);
    let response = if components.len() > 0 {
        response.components(components)
    } else {
        response
    };

    interaction
        .create_response(
            ctx,
            serenity::CreateInteractionResponse::UpdateMessage(response),
        )
        .await?;

    Ok(())
}
