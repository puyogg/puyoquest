use commands::{Data, Error};
use poise::{serenity_prelude as serenity, Framework};

mod util;
mod commands;
mod env_config;
mod embeds;

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
