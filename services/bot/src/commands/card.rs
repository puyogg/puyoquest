use super::{Context, Error};
use crate::embeds::card_embed;
use crate::embeds::character_embed;
use crate::embeds::did_you_mean;
use crate::embeds::CardIconType;
use crate::interaction_router::component::check_if_wiki_editor;
use crate::util::parse_card_query::parse_alias_and_rarity;
use crate::util::parse_card_query::{AliasAndRarity, AliasAndRarityQuery};
use futures::TryFutureExt;
use poise::serenity_prelude::futures::future::try_join_all;
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

    let character = characters_api::characters_get(&data.api_config, Some(&query.fallback)).await?;
    let character = character.get(0);

    match character {
        // Show character embed if card not found
        Some(c) => {
            let (cards_and_materials, aliases) = futures::try_join!(
                sdk::apis::characters_api::characters_id_cards_get(
                    &data.api_config,
                    &c.char_id,
                    Some("false"),
                )
                .map_err(|_| "Failed to fetch character ids".to_string()),
                sdk::apis::aliases_api::aliases_get(&data.api_config, Some(&c.char_id), None, None)
                    .map_err(|_| "Failed to fetch aliases".to_string()),
            )?;
            let is_wiki_editor = check_if_wiki_editor(ctx.serenity_context(), ctx.author()).await;

            let (embed, components) = character_embed(c, &cards_and_materials, &aliases, is_wiki_editor);
            let reply = poise::CreateReply::default()
                .embed(embed)
                .components(components);
            ctx.send(reply).await?;
        }
        // Show suggested characters if character not found
        None => {
            let aliases = sdk::apis::aliases_api::aliases_get(
                &data.api_config,
                None,
                Some(&query.fallback),
                None,
            )
            .await?;

            let character_futures = aliases.iter().map(|a| &a.char_id).map(|char_id| {
                sdk::apis::characters_api::characters_id_get(&data.api_config, char_id)
            });
            let characters = try_join_all(character_futures).await?;

            let (message, component) = did_you_mean(characters, query.fallback);
            let reply = poise::CreateReply::default()
                .content(message)
                .components(vec![component]);

            ctx.send(reply).await?;
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
