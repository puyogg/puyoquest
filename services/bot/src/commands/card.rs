use super::{Context, Error};
use crate::embeds::card_embed;
use crate::embeds::character_embed;
use crate::embeds::CardIconType;
use crate::util::parse_card_query::parse_alias_and_rarity;
use crate::util::parse_card_query::{AliasAndRarity, AliasAndRarityQuery};
use sdk::apis::cards_api;
use sdk::apis::characters_api;

/// Look up a character or card from the PPQ Wiki
#[poise::command(slash_command)]
pub async fn card(
    ctx: Context<'_>,
    #[description = "Look up a character or character card. Ex: Legamunt 7"]
    #[autocomplete = "autocomplete_name"]
    query: String,

    #[description = "The rarity you want to search for if not provided in the first option. (Optional)"]
    rarity: Option<String>,
) -> Result<(), Error> {
    let data = ctx.data();

    let query = match rarity {
        Some(r) => AliasAndRarityQuery {
            query: Some(AliasAndRarity {
                alias: query.clone(),
                rarity: r,
            }),
            fallback: query,
        },
        None => parse_alias_and_rarity(query),
    };

    if let Some(q) = query.query {
        let card = cards_api::cards_get(&data.api_config, Some(&q.alias), Some(&q.rarity)).await;

        match card {
            Ok(c) => {
                let (embed, components) = card_embed(&c, CardIconType::Normal);
                let reply = poise::CreateReply::default().embed(embed);
                let reply = if components.len() > 0 {
                    reply.components(components)
                } else {
                    reply
                };

                ctx.send(reply).await?;

                return Ok(());
            }
            _ => (),
        }
    }

    let alias = characters_api::characters_get(&data.api_config, Some(&query.fallback)).await?;
    let character = alias.get(0);

    match character {
        Some(c) => {
            let cards_and_materials = sdk::apis::characters_api::characters_id_cards_get(
                &data.api_config,
                &c.char_id,
                Some("false"),
            )
            .await?;
            let (embed, components) = character_embed(c, &cards_and_materials);
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

async fn autocomplete_name(ctx: Context<'_>, partial: &str) -> Vec<String> {
    if partial.len() == 0 {
        return vec![];
    }

    let data = ctx.data();
    let api_config = &data.api_config;

    let top_internal_names =
        sdk::apis::aliases_api::aliases_get(api_config, None, Some(partial), None)
            .await
            .map_or(vec![], |aliases| {
                aliases
                    .iter()
                    .map(|a| a.alias.clone())
                    .collect::<Vec<String>>()
            });

    top_internal_names
}
