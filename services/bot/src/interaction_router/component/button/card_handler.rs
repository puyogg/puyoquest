use crate::commands::{Data, Error};
use crate::embeds;
use crate::embeds::CardIconType;
use poise::serenity_prelude as serenity;

/// Expects custom_id in format: {target_embed_type}:{response_type}:{TBD}:{api_id}:{CardIconType}
pub async fn card_handler(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
) -> Result<(), Error> {
    let parsed_custom_id = parse_card_embed_custom_id(&interaction.data.custom_id)?;

    let (card_id, icon_type) = match parsed_custom_id {
        None => {
            interaction
                .create_response(
                    ctx,
                    serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::new()
                            .content("There was an error handling your card request!"),
                    ),
                )
                .await?;

            return Ok(());
        }
        Some(r) => r,
    };

    let card = sdk::apis::cards_api::cards_id_get(&data.api_config, &card_id).await?;

    let (card_embed, components) = embeds::card_embed(&card, icon_type);
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

type CardId = String;
fn parse_card_embed_custom_id(custom_id: &str) -> Result<Option<(CardId, CardIconType)>, Error> {
    // {target_embed_type}:{response_type}:{TBD}:{api_id}:{CardIconType}
    let values = custom_id.split(":").collect::<Vec<&str>>();
    let card_id = values.get(3);
    let icon_type = values.get(4);

    let result = match (card_id, icon_type) {
        (Some(card_id), Some(icon_type)) => {
            let icon_type = match *icon_type {
                "main" => CardIconType::Normal,
                "dual_shift" => CardIconType::DualShift,
                "extra_power" => CardIconType::ExtraPower,
                "extra_power_dual_shift" => CardIconType::ExtraPowerDualShift,
                _ => CardIconType::Normal,
            };
            Some((card_id.to_string(), icon_type))
        }
        _ => None,
    };

    Ok(result)
}

pub fn card_embed_update_custom_id_builder(card_id: &str, icon_type: CardIconType) -> String {
    let icon_type = match icon_type {
        CardIconType::Normal => "main",
        CardIconType::DualShift => "dual_shift",
        CardIconType::ExtraPower => "extra_power",
        CardIconType::ExtraPowerDualShift => "extra_power_dual_shift",
    };

    format!("card:update::{card_id}:{icon_type}")
}
