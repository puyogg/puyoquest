use crate::commands::{Data, Error};
use crate::embeds;
use poise::serenity_prelude as serenity;

pub async fn character_handler(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
) -> Result<(), Error> {
    let custom_id = &interaction.data.custom_id;
    let char_id = parse_character_custom_id(custom_id)?;
    let char_id = match char_id {
        None => {
            interaction
                .create_response(
                    ctx,
                    serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::new()
                            .content("There was an error handling your character request!"),
                    ),
                )
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

type CharId = String;
fn parse_character_custom_id(custom_id: &str) -> Result<Option<CharId>, Error> {
    // character:{response_type}:{TBD}:{char_id}:{TBD}
    let values = custom_id.split(":").collect::<Vec<&str>>();
    let char_id = values.get(3).map(|c| c.to_string());

    Ok(char_id)
}

pub fn character_embed_update_custom_id_builder(char_id: &str) -> String {
    format!("character:update::{char_id}:")
}
