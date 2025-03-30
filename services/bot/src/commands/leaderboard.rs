use poise::CreateReply;

use bot_sdk::apis::default_api as bot_api;

use crate::embeds::leaderboard_embed;

use super::{Context, Error};

#[derive(Debug, poise::ChoiceParameter)]
enum GameType {
    #[name = "ntc"]
    Ntc,
}

impl std::fmt::Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameType::Ntc => f.write_str("ntc"),
        }
    }
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum WhoseScores {
    #[name = "Top 10"]
    Top10,
    Me,
}

#[poise::command(slash_command)]
pub async fn leaderboard(
    ctx: Context<'_>,
    #[description = "Game type"] game_type: GameType,
    #[description = "Whose scores?"] who: WhoseScores,
) -> Result<(), Error> {
    let data = ctx.data();
    let guild_id = ctx.guild_id();
    let guild_id = match guild_id {
        Some(g) => g,
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
    let guild = guild_id.to_partial_guild(ctx).await?;
    let guild_id_string = guild_id.to_string();
    let user_id = ctx.author().id.to_string();

    let rankings = match who {
        WhoseScores::Top10 => {
            let rankings = bot_api::leaderboards_server_id_game_type_top_get(
                &data.bot_api_config,
                &guild_id_string,
                &game_type.to_string(),
            )
            .await;
            match rankings {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("{e}");

                    ctx.send(
                        CreateReply::default()
                            .content("There was an issue fetching the leaderboard. Try again later or contact S2L for help.")
                            .ephemeral(true)
                    ).await?;
                    return Ok(());
                }
            }
        }
        WhoseScores::Me => {
            let rankings = bot_api::leaderboards_server_id_game_type_window_get(
                &data.bot_api_config,
                &guild_id_string,
                &game_type.to_string(),
                &user_id,
            )
            .await;
            match rankings {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("{e}");

                    ctx.send(
                        CreateReply::default()
                            .content("There was an issue fetching the leaderboard. Try again later or contact S2L for help.")
                            .ephemeral(true)
                    ).await?;
                    return Ok(());
                }
            }
        }
    };

    let highlight_user_id = match who {
        WhoseScores::Top10 => Some(user_id.as_str()),
        WhoseScores::Me => None,
    };

    let embed = leaderboard_embed(&ctx, &guild, &who, rankings, highlight_user_id).await;
    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
