use poem::{listener::TcpListener, Result, Server};

use api::init_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = init_api();

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(api)
        .await?;

    Ok(())
}
