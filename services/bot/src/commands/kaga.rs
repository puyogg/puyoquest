use super::{Context, Error};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};

use bot_sdk::apis::default_api::{kaga_get, kaga_random_get};

#[poise::command(slash_command)]
pub async fn kaga(
    ctx: Context<'_>,
    #[description = "Exact ID"]
    #[min_length = 1]
    #[max_length = 160]
    id: Option<String>,
) -> Result<(), Error> {
    let data = ctx.data();

    let kaga_data = match id {
        Some(id) => kaga_get(&data.bot_api_config, &id).await?,
        None => kaga_random_get(&data.bot_api_config).await?,
    };

    let embed = CreateEmbed::default()
        .image(&kaga_data.url)
        .footer(CreateEmbedFooter::new(format!(
            "Kaga Reference id: {}",
            &kaga_data.kaga_id
        )));

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
