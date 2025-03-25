use std::{sync::LazyLock, time::Duration};

use moka::future::Cache;
use poise::{CreateReply, serenity_prelude as serenity};
use utils;

use crate::embeds;

use super::{Context, Error};

pub static active_game_cache: LazyLock<Cache<String, ()>> = LazyLock::new(|| {
    Cache::builder()
        .time_to_live(Duration::from_secs(300))
        .build()
});

#[poise::command(slash_command)]
pub async fn ntc(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();

    let channel_id = ctx.channel_id().to_string();

    let cards = sdk::apis::cards_api::cards_random_card_get(
        &data.api_config,
        1,
        Some(
            ["1239", "2239", "3239", "4239", "5239", "1238"]
                .map(String::from)
                .to_vec(),
        ),
    )
    .await?;
    let card = match cards.get(0) {
        Some(c) => c,
        None => {
            ctx.send(
                CreateReply::default()
                    .content("There was an issue finding a card to show! Try again later.")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    let card_name = &card.name;
    let rarity = match &card.rarity_modifier {
        Some(r) => r,
        None => &card.rarity,
    };
    let jp_name = match &card.jp_name {
        Some(n) => n,
        None => "",
    };

    let full_art =
        sdk::apis::cards_api::cards_card_id_full_art_get(&data.api_config, &card.card_id).await?;

    let aliases: Vec<String> =
        sdk::apis::characters_api::characters_id_aliases_get(&data.api_config, &card.char_id)
            .await?
            .into_iter()
            .map(|a| a.alias)
            .collect();

    let has_active_game = active_game_cache.contains_key(&channel_id);
    if has_active_game {
        ctx.send(
            CreateReply::default()
                .content("There's already an active NTC game in this channel!")
                .ephemeral(true),
        )
        .await?;

        return Ok(());
    }

    active_game_cache.insert(channel_id.clone(), ()).await;

    let embed = embeds::ntc_embed(&card, &full_art);
    ctx.send(
        CreateReply::default()
            .content("Who is this card?")
            .embed(embed),
    )
    .await?;

    while let Some(message) = serenity::MessageCollector::new(ctx)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .await
    {
        let normalized_message = utils::normalize_name(&message.content);

        if aliases.contains(&normalized_message) {
            let user_id = message.author.id.to_string();
            let content = format!(
                "<@{user_id}> got the correct answer!\nThe card was: **{card_name} [★{rarity}] ({jp_name})**"
            );

            ctx.channel_id().say(ctx, content).await?;
            active_game_cache.remove(&channel_id).await;
            return Ok(());
        }
    }

    // timeout
    active_game_cache.remove(&channel_id).await;
    let content = format!("The above card was **{card_name} [★{rarity}] ({jp_name})**");
    ctx.channel_id().say(ctx, content).await?;
    Ok(())
}
