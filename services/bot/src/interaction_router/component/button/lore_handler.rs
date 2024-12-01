use crate::commands::{Data, Error};
use crate::embeds;
use poise::serenity_prelude as serenity;

pub async fn lore_handler(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
) -> Result<(), Error> {
    let custom_id = &interaction.data.custom_id;
    let card_id = parse_lore_embed_custom_id(custom_id)?;

    let card_id = match card_id {
        None => {
            interaction
                .create_response(
                    ctx,
                    serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::new()
                            .content("There was an error handling your lore request!"),
                    ),
                )
                .await?;

            return Ok(());
        }
        Some(c) => c,
    };

    let card = sdk::apis::cards_api::cards_id_get(&data.api_config, &card_id).await?;
    let lore = sdk::apis::cards_api::cards_card_id_lore_get(&data.api_config, &card_id).await?;

    let (embed, components) = embeds::lore_embed(&card, &lore);
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

type CardId = String;
fn parse_lore_embed_custom_id(custom_id: &str) -> Result<Option<CardId>, Error> {
    // {target_embed_type}:{response_type}:{TBD}:{api_id}:{CardIconType}
    let values = custom_id.split(":").collect::<Vec<&str>>();
    let card_id = values.get(3).map(|s| s.to_string());

    Ok(card_id)
}

pub fn lore_nav_button(card_id: &str) -> serenity::CreateButton {
    let custom_id = format!("lore:update::{card_id}:");
    serenity::CreateButton::new(custom_id).label("Lore")
}
