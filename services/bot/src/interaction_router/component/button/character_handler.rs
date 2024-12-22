use crate::commands::{Data, Error};
use crate::embeds;
use crate::interaction_router::component::check_if_wiki_editor;
use crate::util::fetch_character_and_aliases;
use anyhow::anyhow;
use futures::TryFutureExt;
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

    let (character, cards_and_materials, aliases) =
        fetch_character_and_aliases(&data.api_config, &char_id).await?;
    let is_wiki_editor = check_if_wiki_editor(ctx, &interaction.user).await;

    let (embed, components) = embeds::character_embed(&character, &cards_and_materials, &aliases, is_wiki_editor);
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

pub fn character_embed_custom_id(char_id: &str) -> String {
    format!("character:update::{char_id}:")
}

pub fn character_nav_button(char_id: &str) -> serenity::CreateButton {
    let custom_id = character_embed_custom_id(char_id);
    serenity::CreateButton::new(custom_id).label("Character")
}
