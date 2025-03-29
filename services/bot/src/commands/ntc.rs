use std::{sync::LazyLock, time::Duration};

use bot_sdk::apis::default_api as bot_api;
use moka::future::Cache;
use poise::{CreateReply, serenity_prelude as serenity};
use utils;

use crate::embeds;

use super::{Context, Error};

pub static ACTIVE_GAME_CACHE: LazyLock<Cache<String, ()>> = LazyLock::new(|| {
    Cache::builder()
        .time_to_live(Duration::from_secs(300))
        .build()
});

#[poise::command(slash_command)]
pub async fn ntc(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();

    let channel_id = ctx.channel_id().to_string();
    let guild_id = match ctx.guild_id() {
        Some(g) => g.to_string(),
        None => {
            ctx.send(
                CreateReply::default()
                    .content("There was an issue fetching your server. Try again later.")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

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

    let has_active_game = ACTIVE_GAME_CACHE.contains_key(&channel_id);
    if has_active_game {
        ctx.send(
            CreateReply::default()
                .content("There's already an active NTC game in this channel!")
                .ephemeral(true),
        )
        .await?;

        return Ok(());
    }

    ACTIVE_GAME_CACHE.insert(channel_id.clone(), ()).await;

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
        let user_id = message.author.id.to_string();
        let normalized_message = utils::normalize_name(&message.content);

        if aliases.contains(&normalized_message) {
            let score = bot_api::leaderboards_server_id_game_type_user_id_increment_post(
                &data.bot_api_config,
                &guild_id,
                "ntc",
                &user_id,
            )
            .await;

            let score = match score {
                Ok(score) => score.correct,
                Err(_) => {
                    ctx.send(CreateReply::default().content(
                        "There was an issue updating your score. Contact S2 or try again later.",
                    ))
                    .await?;
                    return Ok(());
                }
            };

            let content = format!(
                r#"<@{user_id}> got the correct answer!
The card was: **{card_name} [★{rarity}] ({jp_name})**

Your total score is: {score}"#
            );

            ctx.channel_id().say(ctx, content).await?;
            ACTIVE_GAME_CACHE.remove(&channel_id).await;
            return Ok(());
        }
    }

    // timeout
    ACTIVE_GAME_CACHE.remove(&channel_id).await;
    let content = format!("The above card was **{card_name} [★{rarity}] ({jp_name})**");
    ctx.channel_id().say(ctx, content).await?;
    Ok(())
}
