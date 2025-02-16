use std::sync::LazyLock;

use crate::env_config;

use super::{Context, Error};
use wiki_indexers::card_indexer::indexer::CardIndexer;

static INDEXER: LazyLock<CardIndexer> =
    LazyLock::new(|| CardIndexer::new(&env_config::ENV.ppq_api_host));

#[poise::command(slash_command)]
pub async fn reindex(ctx: Context<'_>, char_id: String) -> Result<(), Error> {
    let env = &*env_config::ENV;

    let guild_id = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => {
            ctx.reply("Failed to find guild_id").await?;
            return Ok(());
        }
    };

    let is_eppc = guild_id == env.primary_server_id;
    if !is_eppc {
        ctx.reply("You can only use this command in the EPPC Discord server.")
            .await?;
        return Ok(());
    }

    let is_wiki_editor = ctx
        .author()
        .has_role(ctx, guild_id, env.wiki_editor_role_id)
        .await?;

    if !is_wiki_editor {
        ctx.reply("You must be a Wiki Editor to use this command.")
            .await?;
        return Ok(());
    }

    let result = INDEXER.index_char(&char_id).await;

    match result {
        Ok(r) => {
            ctx.reply(format!(
                "Successfully indexed char_id {}: {}",
                &char_id,
                r.character.name.unwrap_or("(Unknown Name)".to_string()),
            ))
            .await?;
        }
        Err(e) => {
            ctx.reply(format!("Error indexing char_id: {}", &char_id))
                .await?;
            println!("{e}");
        }
    };

    Ok(())
}
