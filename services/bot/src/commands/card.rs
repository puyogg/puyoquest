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
                let c_text = serde_json::to_string(&c);
                match c_text {
                    Ok(c) => {
                        let json = serde_json::value::to_raw_value(&c)?;
                        let pretty = serde_json::to_string_pretty(&json)?;

                        ctx.say(format!(
                            "```\n{}\n```",
                            &c
                        ))
                        .await?;

                        ctx.send(poise::CreateReply::default()
                            .content("OK?")
                            .embed(card_embed())
                        ).await?;
                        return Ok(());
                    }
                    Err(e) => {
                        ctx.say(format!("Failed to serialize Card: {}", e)).await?;
                        return Ok(());
                    }
                };
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
