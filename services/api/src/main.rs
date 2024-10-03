use poem::{listener::TcpListener, Result, Server};

use api::db::create_pool;
use api::init_api;
use api::cache::create_redis_connection;
use wiki::wiki_client::WikiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_pool().await?;
    let wiki_client = WikiClient::new(
        "https://puyonexus.com/mediawiki/api.php",
        "https://puyonexus.com/wiki",
    );

    let redis_conn = create_redis_connection("0.0.0.0", "36379").await;

    let api = init_api(pool, wiki_client, redis_conn);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(api)
        .await?;

    Ok(())
}
