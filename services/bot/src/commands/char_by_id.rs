// use crate::types::{Context, Error};
// use crate::types::{Context, Error};
use super::{Context, Error};

#[poise::command(slash_command)]
pub async fn char_by_id(ctx: Context<'_>, id: Option<String>) -> Result<(), Error> {
    println!("{:?}", ctx.data());
    // sdk::apis::characters_api::characters_id_get(configuration, id);
    let id = id.unwrap_or("NO ID".to_string());
    ctx.reply(&format!("Requested char id: {id}")).await?;
    Ok(())
}
