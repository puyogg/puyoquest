use crate::embeds::lore_embed;

use super::{Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};
use rand::seq::SliceRandom;

#[poise::command(slash_command)]
pub async fn whoselore(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();

    let current_channel_id = ctx.channel_id().to_string();
    if data.active_lore_game.contains(&current_channel_id) {
        ctx.send(
            CreateReply::default()
                .content("There's already an active game in this channel!")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    // TODO: lore fetch could error
    let lore = sdk::apis::cards_api::cards_random_lore_get(&data.api_config).await?;
    let card = sdk::apis::cards_api::cards_id_get(&data.api_config, &lore.card_id).await?;
    let aliases =
        sdk::apis::characters_api::characters_id_aliases_get(&data.api_config, &card.char_id)
            .await?;
    let alias_strings: Vec<String> = aliases.iter().map(|a| a.alias.clone()).collect();
    println!("{:?}", alias_strings);

    let embed = serenity::CreateEmbed::default().title("Whose lore is this?");

    let mut possible_quotes: Vec<(String, String)> = Vec::new();
    if let Some(flavor_text) = &lore.flavor_text_en {
        possible_quotes.push(("Flavor Text".to_string(), flavor_text.to_string()));
    }
    for line in lore.clone().monologue_lines {
        if let Some(l) = line.en {
            possible_quotes.push(("Monologue Line".to_string(), l));
        }
    }

    let quote = possible_quotes.choose(&mut rand::thread_rng());
    let embed = match quote {
        None => {
            ctx.say("There was an error fetching a lore quote.").await?;
            return Ok(());
        }
        Some(q) => embed.field(q.0.clone(), q.1.clone(), false),
    };

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    data.active_lore_game.insert(current_channel_id);

    let expected_rarity = match &card.rarity_modifier {
        Some(rm) => rm.clone(),
        None => card.rarity.clone(),
    };

    let translator_and_editor = match (&lore.translator, &lore.editor) {
        (Some(t), Some(e)) => Some(format!("{}, {}", t, e)),
        (Some(t), None) => Some(t.clone()),
        _ => None,
    };
    let thank_you_text = translator_and_editor
        .map(|t| format!("Thank you to {t} for translating!"))
        .unwrap_or("".to_string());

    println!("{} {}", &card.name, &expected_rarity);

    while let Some(mci) = serenity::MessageCollector::new(ctx)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(45))
        .await
    {
        let guess = &mci.content.to_lowercase();

        let exact_guess = parse_guess(&guess);

        if let Some((maybe_name, maybe_rarity)) = exact_guess {
            if alias_strings.contains(&maybe_name) && maybe_rarity == expected_rarity {
                if maybe_rarity == expected_rarity {
                    data.active_lore_game.remove(&ctx.channel_id().to_string());
                    let user_id = mci.author.id.to_string();
                    let current_score = data.lore_score.get(&user_id);
                    let updated_score = match current_score {
                        Some(score) => score.clone() + 3,
                        None => 3,
                    };
                    data.lore_score.insert(user_id, updated_score.clone());

                    mci.reply_mention(
                        ctx,
                        format!(
                            r#"Perfect! This quote is from: {} ★{}. {}
    You earned 3 points. Your total score is **{} points.**"#,
                            &card.name, expected_rarity, thank_you_text, updated_score
                        ),
                    )
                    .await?;

                    let (embed, components) = lore_embed(&card, &lore);
                    let lore_tip = serenity::CreateMessage::default().embed(embed);
                    let lore_tip = if components.len() > 0 {
                        lore_tip.components(components)
                    } else {
                        lore_tip
                    };

                    let channel = ctx.channel_id();
                    channel.send_message(ctx, lore_tip).await?;
                    return Ok(());
                }

                // Give credit for guessing the right name, but wrong rarity
                data.active_lore_game.remove(&ctx.channel_id().to_string());
                let user_id = mci.author.id.to_string();
                let current_score = data.lore_score.get(&user_id);
                let updated_score = match current_score {
                    Some(score) => score.clone() + 1,
                    None => 1,
                };
                data.lore_score.insert(user_id, updated_score.clone());

                mci.reply_mention(
                    ctx,
                    format!(
                        r#"Close! This quote is from: {} ★{}. {}
    You earned 1 point. Your total score is **{} points.**"#,
                        &card.name, expected_rarity, thank_you_text, updated_score
                    ),
                )
                .await?;

                let (embed, components) = lore_embed(&card, &lore);
                let lore_tip = serenity::CreateMessage::default().embed(embed);
                let lore_tip = if components.len() > 0 {
                    lore_tip.components(components)
                } else {
                    lore_tip
                };

                let channel = ctx.channel_id();
                channel.send_message(ctx, lore_tip).await?;
                return Ok(());
            }
        }

        if alias_strings.contains(&guess) {
            data.active_lore_game.remove(&ctx.channel_id().to_string());
            let user_id = mci.author.id.to_string();
            let current_score = data.lore_score.get(&user_id);
            let updated_score = match current_score {
                Some(score) => score.clone() + 1,
                None => 1,
            };
            data.lore_score.insert(user_id, updated_score.clone());

            mci.reply_mention(
                ctx,
                format!(
                    r#"Nice! This quote is from: {} ★{}. {}
You earned 1 point. Your total score is **{} points.**"#,
                    &card.name, expected_rarity, thank_you_text, updated_score
                ),
            )
            .await?;

            let (embed, components) = lore_embed(&card, &lore);
            let lore_tip = serenity::CreateMessage::default().embed(embed);
            let lore_tip = if components.len() > 0 {
                lore_tip.components(components)
            } else {
                lore_tip
            };

            let channel = ctx.channel_id();
            channel.send_message(ctx, lore_tip).await?;
            return Ok(());
        }
    }

    data.active_lore_game.remove(&ctx.channel_id().to_string());

    let (embed, components) = lore_embed(&card, &lore);
    let lore_tip = serenity::CreateMessage::default()
        .embed(embed)
        .content(format!(
            "Time's up! The quote is from: {} ★{}",
            &card.name, expected_rarity
        ));
    let lore_tip = if components.len() > 0 {
        lore_tip.components(components)
    } else {
        lore_tip
    };

    let channel = ctx.channel_id();
    channel.send_message(ctx, lore_tip).await?;
    Ok(())
}

fn parse_guess(guess: &str) -> Option<(String, String)> {
    let mut parts: Vec<&str> = guess.split(" ").collect();
    let last = parts.pop();
    match last {
        Some(maybe_rarity) => {
            let maybe_rarity = match maybe_rarity.to_lowercase().as_ref() {
                "6s" => "6-2".to_string(),
                _ => maybe_rarity.to_string(),
            };

            let maybe_name = parts.join(" ").trim().to_string();
            Some((maybe_name, maybe_rarity))
        }
        None => None,
    }
}
