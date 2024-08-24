use std::{env, vec};
// use std::sync::atomic::{AtomicU32, Ordering};
use commands::{char_by_id::char_by_id, Data, Error};
use poise::{serenity_prelude as serenity, Framework};

mod commands;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN env var");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework: Framework<Data, Error> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![char_by_id()],
            ..Default::default()
        })
        .setup(move |ctx, ready, framework: &poise::Framework<Data, _>| {
            Box::pin(async move {
                println!("{:?}", ready);
                poise::builtins::register_in_guild(ctx, &framework.options().commands, serenity::GuildId::new(133012933260214272)).await?;
                Ok(Data {
                    hello: "world!".to_string(),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
