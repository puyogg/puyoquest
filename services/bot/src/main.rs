use aws::AwsClient;
use commands::{Data, Error};
use dashmap::{DashMap, DashSet};
use poise::{Framework, serenity_prelude as serenity};

mod aws;
mod commands;
mod embeds;
mod env_config;
mod interaction_router;
mod util;

#[tokio::main]
async fn main() {
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let env = &*env_config::ENV;

    let _ = tracing_subscriber::fmt::init();

    let guild_id = env.primary_server_id.clone();
    let api_config = (&*env_config::API_CONFIG).clone();
    let bot_api_config = (&*env_config::BOT_API_CONFIG).clone();
    let sdk_config = aws_config::from_env().load().await;
    let aws_client = AwsClient::new(sdk_config);

    let commands = std::vec![
        commands::char_by_id::char_by_id(),
        commands::card::card(),
        commands::whoselore::whoselore(),
        commands::categorysearch::categorysearch(),
        commands::incorrect_quote::iq(),
        commands::ppq_events::ppqevents(),
        commands::reindex::reindex(),
        commands::server_settings::server_settings(),
    ];

    let framework: Framework<Data, Error> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(move |ctx, ready, framework: &poise::Framework<Data, _>| {
            Box::pin(async move {
                println!("{:?}", ready);
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    serenity::GuildId::new(guild_id),
                )
                .await?;
                Ok(Data {
                    api_config,
                    aws_client,
                    active_lore_game: DashSet::new(),
                    lore_score: DashMap::new(),
                    bot_api_config,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(&env.bot_token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
            let guilds = ctx
                .cache
                .guilds()
                .iter()
                .map(|g| g.to_string())
                .collect::<Vec<String>>();
            tracing::info!("Current guilds: {:?}", guilds);
        }
        serenity::FullEvent::InteractionCreate { interaction } => {
            interaction_router::interaction_router(ctx, data, interaction).await?;
        }
        serenity::FullEvent::GuildCreate { guild, is_new } => {
            if let Some(is_new) = is_new {
                if !is_new {
                    return Ok(());
                }
            }

            let settings = bot_sdk::apis::default_api::server_settings_post(
                &data.bot_api_config,
                bot_sdk::models::ServerSettings {
                    server_id: guild.id.to_string(),
                },
            )
            .await;

            match settings {
                Ok(s) => {
                    tracing::info!(
                        "Created settings for new server: {} ({})",
                        &guild.name,
                        s.server_id
                    );
                }
                Err(e) => {
                    tracing::error!(
                        "Error creating settings for new server: {} ({}). {:?}",
                        &guild.name,
                        &guild.id,
                        e
                    );
                }
            };
        }
        serenity::FullEvent::GuildDelete { incomplete, full } => {
            let result = bot_sdk::apis::default_api::server_settings_server_id_delete(
                &data.bot_api_config,
                &incomplete.id.to_string(),
            )
            .await;

            match result {
                Ok(_) => {
                    tracing::info!("Deleted settings for guild: {}", &incomplete.id);
                }
                Err(e) => {
                    tracing::error!("Failed to delete guild: {}. {:?}", &incomplete.id, e);
                }
            };
        }
        _ => {}
    }

    Ok(())
}
