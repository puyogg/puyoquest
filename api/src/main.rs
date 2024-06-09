use poem::{listener::TcpListener, Result, Server};

use api::init_api;
use api::db::create_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_pool().await?;
    let api = init_api(pool);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(api)
        .await?;

    Ok(())
}
