use super::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::seq::SliceRandom;

#[poise::command(slash_command)]
pub async fn whoselore(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();

    // TODO: lore fetch could error
    let lore = sdk::apis::cards_api::cards_random_lore_get(&data.api_config).await?;
    let card = sdk::apis::cards_api::cards_id_get(&data.api_config, &lore.card_id).await?;
    let aliases =
        sdk::apis::characters_api::characters_id_aliases_get(&data.api_config, &card.char_id)
            .await?;

    let embed = serenity::CreateEmbed::default().title("Whose lore is this?");

    let mut possible_quotes: Vec<(String, String)> = Vec::new();
    if let Some(flavor_text) = lore.flavor_text_en {
        possible_quotes.push(("Flavor Text".to_string(), flavor_text));
    }
    for line in lore.monologue_lines {
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
    Ok(())
}
