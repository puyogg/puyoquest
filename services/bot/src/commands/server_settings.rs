use super::{Context, Error};
use poise::CreateReply;

use bot_sdk::apis::default_api::{
    leaderboard_channel_server_id_game_type_delete, leaderboard_channel_server_id_game_type_get,
    leaderboard_channel_server_id_game_type_post,
};

#[derive(Debug, poise::ChoiceParameter)]
pub enum Crud {
    Get,
    Set,
    Delete,
}

const GAME_TYPE: &'static str = "ntc";

#[poise::command(
    slash_command,
    subcommands("ntc_leaderboard_channel"),
    subcommand_required,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn server_settings(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn ntc_leaderboard_channel(
    ctx: Context<'_>,
    #[description = "Operation"] operation: Crud,
) -> Result<(), Error> {
    let data = ctx.data();
    let guild_id = ctx.guild_id();

    let guild_id = match guild_id {
        Some(g) => g.to_string(),
        None => {
            ctx.send(
                CreateReply::default()
                    .reply(true)
                    .ephemeral(true)
                    .content("Guild unavailable"),
            )
            .await?;
            return Ok(());
        }
    };

    match operation {
        Crud::Get => {
            let channel_id = leaderboard_channel_server_id_game_type_get(
                &data.bot_api_config,
                &guild_id,
                GAME_TYPE,
            )
            .await?
            .channel_id;

            ctx.send(
                CreateReply::default()
                    .reply(true)
                    .ephemeral(true)
                    .content(format!("The current ntc leaderboard channel is set to: <#{channel_id}> ({channel_id})"))
            ).await?;
        }
        Crud::Set => {
            let channel_id = leaderboard_channel_server_id_game_type_post(
                &data.bot_api_config,
                &guild_id,
                GAME_TYPE,
            )
            .await?
            .channel_id;

            ctx.send(
                CreateReply::default()
                    .reply(true)
                    .ephemeral(true)
                    .content(format!("Successfully set the ntc leaderboard channel to: <#{channel_id}> ({channel_id})"))
            ).await?;
        }
        Crud::Delete => {
            let result = leaderboard_channel_server_id_game_type_delete(
                &data.bot_api_config,
                &guild_id,
                GAME_TYPE,
            )
            .await;

            match result {
                Ok(_) => {
                    ctx.send(
                        CreateReply::default()
                            .reply(true)
                            .ephemeral(true)
                            .content(format!("Successfully unset the ntc leaderboard channel")),
                    )
                    .await?;
                }
                Err(_) => {
                    ctx.send(
                        CreateReply::default()
                            .reply(true)
                            .ephemeral(true)
                            .content(format!("There was an issue unsetting the ntc leaderboard channel. Try again later."))
                    ).await?;
                }
            }
        }
    };

    Ok(())
}
