use super::{Context, Error};

/// Look up a character or card from the PPQ Wiki
#[poise::command(slash_command)]
pub async fn card(
    ctx: Context<'_>,
    #[description = "Look up a character or character card. Ex: Legamunt 7"] query: String,
) -> Result<(), Error> {
    ctx.say(format!("You requested: {query}")).await?;

    Ok(())
}
