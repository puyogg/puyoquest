use std::sync::{Arc, LazyLock};

use discord_logger::discord_logger;
use tokio::sync::Mutex;

static WEBHOOK_URL: LazyLock<Option<String>> = LazyLock::new(|| {
    let webhook_url = std::env::var("DISCORD_LOG_WEBHOOK_URL");
    match webhook_url {
        Ok(url) => Some(url),
        Err(_) => {
            println!("WARNING! Invalid or missing env var DISCORD_LOG_WEBHOOK_URL. Messages will not be logged to your Discord channel.");
            return None;
        }
    }
});

#[tokio::main]
async fn main() {
    let (logger, listener) = discord_logger(WEBHOOK_URL.clone().unwrap());
    let logger = Box::new(logger);
    log::set_boxed_logger(logger).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let listener = Arc::new(Mutex::new(listener));

    let handle = tokio::spawn(async move {
        listener.lock().await.listen();
    });

    log::info!("Test!!!");
    handle.await.unwrap();
}
