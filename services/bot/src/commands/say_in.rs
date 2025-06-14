use super::{Context, Error};
use poise::serenity_prelude as serenity;

/// Chat as Yotarou
#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn say_in(
    ctx: Context<'_>,
    channel: serenity::GuildChannel,
    message: String,
) -> Result<(), Error> {
    channel.say(ctx, message).await?;
    Ok(())
}
