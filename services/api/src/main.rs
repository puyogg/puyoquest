use api::aws::s3::S3BackupClient;
use api::config::ApiConfig;
use poem::{listener::TcpListener, Result, Server};

use api::cache::create_redis_connection;
use api::db::create_pool;
use api::{env_config, init_api};
use wiki::wiki_client::WikiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = &*env_config::ENV;

    let _install_default = rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();

    let pool = create_pool(
        &env.environment,
        &env.db_connection_string,
    ).await?;
    let wiki_client = WikiClient::new(&env.pn_wiki_api_url, &env.pn_wiki_base_url);

    let redis_conn = std::sync::Arc::new(
        create_redis_connection(
            &env.environment,
            &env.redis_connection_string,
            env.redis_key_prefix.clone(),
        )
        .await,
    );

    let api_config = std::sync::Arc::new(ApiConfig::new().await?);
    let s3_backup_client = std::sync::Arc::new(S3BackupClient::new().await);

    let api = init_api(api_config, pool, wiki_client, redis_conn, s3_backup_client);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(api)
        .await?;

    Ok(())
}
