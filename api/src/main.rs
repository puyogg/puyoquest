use poem::{listener::TcpListener, Result, Server};

use api::db::create_pool;
use api::init_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_pool().await?;
    let api = init_api(
        pool,
        String::from("https://puyonexus.com/mediawiki/api.php"),
        String::from("https://puyonexus.com/wiki"),
    );

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(api)
        .await?;

    Ok(())
}
