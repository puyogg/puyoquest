use commands::{Data, Error};
use poise::{serenity_prelude as serenity, Framework};

mod util;
mod commands;
mod env_config;
mod embeds;
mod interaction_router;

#[tokio::main]
async fn main() {
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let env = &*env_config::ENV;
    let guild_id = env.primary_server_id.clone();
    let api_config = (&*env_config::API_CONFIG).clone();

    let commands = std::vec![
        commands::char_by_id::char_by_id(),
        commands::card::card(),
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
                    hello: "world!".to_string(),
                    api_config,
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
        },
        serenity::FullEvent::InteractionCreate { interaction } => {
            interaction_router::interaction_router(ctx, data, interaction).await?;
        },
        _ => {},
    }

    Ok(())
}
