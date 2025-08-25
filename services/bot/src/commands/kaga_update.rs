use super::{Context, Error};
use bot_sdk::models::KagaData;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};

use bot_sdk::apis::default_api::{kaga_delete, kaga_put};

#[poise::command(
    slash_command,
    subcommands("kaga_put_command", "kaga_delete_command"),
    subcommand_required,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn kaga_update(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
/// Set a new kaga image
async fn kaga_put_command(
    ctx: Context<'_>,
    #[description = "Kaga ID"]
    #[min_length = 1]
    #[max_length = 160]
    kaga_id: String,
    #[description = "Image URL"] url: String,
) -> Result<(), Error> {
    let data = ctx.data();

    let kaga_data = KagaData { kaga_id, url };

    let result = kaga_put(&data.bot_api_config, kaga_data).await;

    match result {
        Ok(k) => {
            let embed = CreateEmbed::default()
                .image(&k.url)
                .footer(CreateEmbedFooter::new(format!(
                    "Kaga Reference id: {}",
                    &k.kaga_id
                )));

            let reply = poise::CreateReply::default()
                .content(format!("Successfully set Kaga ID: {}", k.kaga_id))
                .embed(embed);

            ctx.send(reply).await?;
        }
        Err(e) => {
            tracing::error!("{:?}", e);

            ctx.reply("Failed to set Kaga Image. Contact S2L!").await?;
        }
    };

    Ok(())
}

#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
/// Delete a kaga image by id
async fn kaga_delete_command(
    ctx: Context<'_>,
    #[description = "Kaga ID to delete"]
    #[min_length = 1]
    #[max_length = 160]
    kaga_id: String,
) -> Result<(), Error> {
    let data = ctx.data();

    let result = kaga_delete(&data.bot_api_config, &kaga_id).await;

    match result {
        Ok(_) => {
            ctx.reply(&format!("Successfully deleted kaga id: {kaga_id}"))
                .await?;
        }
        Err(e) => {
            tracing::error!("{:?}", e);

            ctx.reply("Failed to delete Kaga Image. Contact S2L!")
                .await?;
        }
    };

    Ok(())
}
