use api::aws::s3::S3BackupClient;
use api::config::ApiConfig;
use poem::{listener::TcpListener, Result, Server};

use api::cache::{create_redis_connection, DEFAULT_REDIS_KEY_PREFIX};
use api::db::create_pool;
use api::init_api;
use wiki::wiki_client::WikiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_pool().await?;
    let wiki_client = WikiClient::new(
        "https://puyonexus.com/mediawiki/api.php",
        "https://puyonexus.com/wiki",
    );

    let redis_conn = std::sync::Arc::new(
        create_redis_connection("0.0.0.0", "36379", DEFAULT_REDIS_KEY_PREFIX.to_string()).await,
    );

    let api_config = std::sync::Arc::new(ApiConfig::new().await?);
    let s3_backup_client = std::sync::Arc::new(S3BackupClient::new().await);

    let api = init_api(api_config, pool, wiki_client, redis_conn, s3_backup_client);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(api)
        .await?;

    Ok(())
}
