use super::{Context, Error};
use crate::embeds::card_embed;
use crate::embeds::character_embed;
use crate::embeds::CardIconType;
use crate::util::parse_card_query::parse_alias_and_rarity;
use sdk::apis::cards_api;
use sdk::apis::characters_api;

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
                let (embed, components) = card_embed(&c, CardIconType::Normal).await?;
                let reply = poise::CreateReply::default().embed(embed);
                let reply = if components.len() > 0 {
                    reply.components(components)
                } else {
                    reply
                };

                ctx.send(reply).await?;

                return Ok(());
            }
            Err(e) => {
                ctx.say("Failed to find card; falling back to character query")
                    .await?;
            }
        }
    }

    let alias = characters_api::characters_get(&data.api_config, Some(&query.fallback)).await?;
    let character = alias.get(0);

    match character {
        Some(c) => {
            let (embed, components) = character_embed(&data.api_config, c).await?;
            let reply = poise::CreateReply::default()
                .embed(embed)
                .components(components);
            ctx.send(reply).await?;
        }
        None => {
            ctx.say(format!("Failed to find character: {}", &query.fallback))
                .await?;
        }
    }

    Ok(())
}
