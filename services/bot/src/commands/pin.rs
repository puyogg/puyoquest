use poise::CreateReply;

use super::{Context, Error};

#[poise::command(slash_command)]
pub async fn pin(
    ctx: Context<'_>,
    #[description = "The URL or ID of a channel message"] message_id: String,
) -> Result<(), Error> {
    let message_id = message_id.split("/").last();
    let message_id = match message_id {
        Some(m) => {
            let id = m.parse::<u64>();
            match id {
                Ok(id) => id,
                Err(e) => {
                    tracing::error!("{e}");
                    ctx.send(
                        CreateReply::default()
                            .ephemeral(true)
                            .content("Invalid message id"),
                    )
                    .await?;
                    return Ok(());
                }
            }
        }
        None => {
            ctx.send(
                CreateReply::default()
                    .ephemeral(true)
                    .content("Failed to find the given message."),
            )
            .await?;
            return Ok(());
        }
    };

    let guild_channel = ctx.guild_channel().await;
    let guild_channel = match guild_channel {
        Some(g) => g,
        None => {
            ctx.send(
                CreateReply::default()
                    .ephemeral(true)
                    .content("Failed to find the guild the message is in."),
            )
            .await?;
            return Ok(());
        }
    };
    let message = guild_channel.message(ctx, message_id).await?;

    if message.pinned {
        message.unpin(ctx).await?;
    } else if !message.pinned {
        message.pin(ctx).await?;
    }

    Ok(())
}
