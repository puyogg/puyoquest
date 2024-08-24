// use crate::types::{Context, Error};
// use crate::types::{Context, Error};
use super::{Context, Error};

#[poise::command(slash_command)]
pub async fn char_by_id(ctx: Context<'_>) -> Result<(), Error> {
    println!("{:?}", ctx.data());
    // sdk::apis::characters_api::characters_id_get(configuration, id);
    ctx.reply("Test").await?;
    Ok(())
}
