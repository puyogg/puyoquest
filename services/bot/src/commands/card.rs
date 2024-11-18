use super::{Context, Error};
use crate::{commands::Data, util::parse_card_query::parse_alias_and_rarity};
use sdk::apis::cards_api;
use serde::Serialize;
use crate::embeds::card_embed;

/// Look up a character or card from the PPQ Wiki
#[poise::command(slash_command)]
pub async fn card(
    ctx: Context<'_>,
    #[description = "Look up a character or character card. Ex: Legamunt 7"] query: String,
) -> Result<(), Error> {
    // ctx.say(format!("You requested: {query}")).await?;
    let data = ctx.data();

    let query = parse_alias_and_rarity(query);

    if let Some(q) = query.query {
        let card = cards_api::cards_get(&data.api_config, Some(&q.alias), Some(&q.rarity)).await;

        match card {
            Ok(c) => {
                let embed = card_embed(&c).await?;
                ctx.send(poise::CreateReply::default()
                    .embed(embed)
                ).await?;

                return Ok(());
            }
            Err(e) => {
                ctx.say("Failed to find card; falling back to character query")
                    .await?;
            }
        }
    }

    ctx.say("Gotta look up character...").await?;

    Ok(())
}
