use poem::{listener::TcpListener, Result, Server};

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

    let api = init_api(pool, wiki_client);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(api)
        .await?;

    Ok(())
}
