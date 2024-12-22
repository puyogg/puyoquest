use crate::{
    commands::{Data, Error},
    env_config::ENV,
};
use poise::serenity_prelude::{self as serenity, ComponentInteraction};

#[derive(Debug)]
pub enum PermissionError {
    MissingOriginalInteraction,
    NotOriginalUser,
    NotWikiEditor,
    UnknownDiscordError,
}
impl std::error::Error for PermissionError {}
impl std::fmt::Display for PermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionError::MissingOriginalInteraction => {
                write!(f, "Missing original interaction")
            }
            PermissionError::NotOriginalUser => {
                write!(f, "Not the original interaction user")
            }
            PermissionError::NotWikiEditor => {
                write!(f, "Not a wiki editor")
            }
            PermissionError::UnknownDiscordError => write!(f, "Unknown serenity error!"),
        }
    }
}

fn _check_from_original_user(
    interaction: &serenity::model::application::ComponentInteraction,
) -> Result<(), PermissionError> {
    let original_interaction = &interaction.message.interaction;
    let original_user = match original_interaction {
        None => return Err(PermissionError::MissingOriginalInteraction),
        Some(i) => &i.user,
    };

    let calling_user = &interaction.user;

    if original_user.id == calling_user.id {
        Ok(())
    } else {
        Err(PermissionError::NotOriginalUser)
    }
}

pub async fn check_from_original_user(
    ctx: &serenity::Context,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    let result = _check_from_original_user(interaction);

    match result {
        Ok(_) => Ok(()),
        Err(e) => handle_permission_error(ctx, interaction, e).await,
    }
}

/// Responds to interaction, then rethrows
async fn handle_permission_error(
    ctx: &serenity::Context,
    interaction: &ComponentInteraction,
    error: PermissionError,
) -> Result<(), Error> {
    let response_message = serenity::CreateInteractionResponseMessage::new().ephemeral(true);

    let response_message = match error {
        PermissionError::MissingOriginalInteraction => response_message
            .content("Interaction Failed! Missing the original message interaction."),
        PermissionError::NotOriginalUser => response_message.content(
            "This embed doesn't belong to you! Use slash commands to make your own, e.g. /card",
        ),
        PermissionError::NotWikiEditor => response_message
            .content("Sorry! You need to be an EPPC Wiki Editor to perform this action."),
        PermissionError::UnknownDiscordError => response_message
            .content("Sorry, an unknown Discord API error happened. Try again later."),
    };

    interaction
        .create_response(
            ctx,
            serenity::CreateInteractionResponse::Message(response_message),
        )
        .await?;

    Err(Box::new(error))
}

pub async fn check_if_wiki_editor(
    ctx: &serenity::Context,
    user: &serenity::User,
) -> bool {
    let env = &*ENV;
    let guild_id = env.primary_server_id;
    let wiki_editor_role_id = env.wiki_editor_role_id;
    let has_role_result = user
        .has_role(ctx, guild_id, wiki_editor_role_id)
        .await;

    match has_role_result {
        Ok(h) => h,
        Err(_) => false,
    }
}

pub async fn check_if_wiki_editor_and_error(
    ctx: &serenity::Context,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    let env = &*ENV;
    let guild_id = env.primary_server_id;
    let wiki_editor_role_id = env.wiki_editor_role_id;
    let has_role = interaction
        .user
        .has_role(ctx, guild_id, wiki_editor_role_id)
        .await
        .map_err(|_| PermissionError::UnknownDiscordError)?;

    match has_role {
        true => return Ok(()),
        false => handle_permission_error(ctx, interaction, PermissionError::NotWikiEditor).await,
    }
}
