use commands::{char_by_id::char_by_id, Data, Error};
use poise::{serenity_prelude as serenity, Framework};

mod util;
mod commands;
mod env_config;

#[tokio::main]
async fn main() {
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let env = env_config::load_env_file().unwrap();
    let guild_id = env.primary_server_id.clone();

    let framework: Framework<Data, Error> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: std::vec![char_by_id()],
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
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(env.bot_token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
