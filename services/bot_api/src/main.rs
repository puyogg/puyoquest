use std::sync::Arc;

use bot_api::{db::create_pool, env::load_env, init_api};
use poem::{Server, listener::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // let _install_default = rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();
    let sdk_config = aws_config::from_env().load().await;
    let aws_client = bot_api::aws::AwsClient::new(sdk_config);

    let env = load_env(&aws_client)
        .await
        .expect("env vars or secrets manager should be configured");

    let pool = create_pool(&env.environment, &env.db_connection_string).await?;

    let aws_client = Arc::new(aws_client);

    let api = init_api(aws_client, pool);
    Server::new(TcpListener::bind(format!("0.0.0.0:{}", env.bot_api_port)))
        .run(api)
        .await?;

    Ok(())
}
