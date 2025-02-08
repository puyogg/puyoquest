use std::sync::mpsc::{self, Receiver, Sender};
use std::{str::FromStr, sync::LazyLock};

pub use log::Level;
use serde_json::{json, Value};
use std::time::Duration;
use ureq::Agent;

static RUST_LOG: LazyLock<Option<String>> = LazyLock::new(|| std::env::var("RUST_LOG").ok());

pub struct DiscordLogger {
    sender: Sender<Value>,
    log_level: Level,
}

impl DiscordLogger {
    /// Uses the RUST_LOG environment variable if it exists. Falls back to Level::Info if not.
    pub fn new(sender: Sender<Value>) -> DiscordLogger {
        let log_level = RUST_LOG
            .as_ref()
            .map(|l| Level::from_str(&l).ok())
            .flatten()
            .unwrap_or(Level::Info);

        DiscordLogger { sender, log_level }
    }

    pub fn with_level(sender: Sender<Value>, log_level: Level) -> DiscordLogger {
        DiscordLogger { sender, log_level }
    }
}

impl log::Log for DiscordLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let body = json!({
                "content": format!("[{}] {}", record.level(), record.args())
            });

            let send_result = self.sender.send(body);

            if let Err(e) = send_result {
                println!("WARNING! There was an issue sending a message to the logging thread.");
                println!("{:?}", e);
            }
        }
    }

    fn flush(&self) {}
}

pub struct DiscordLogListener {
    receiver: Receiver<Value>,
    client: Agent,
    url: String,
}

impl DiscordLogListener {
    pub fn new(recv: Receiver<Value>, webhook_url: String) -> DiscordLogListener {
        let config = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(5)))
            .build();

        let client: Agent = config.into();

        DiscordLogListener {
            receiver: recv,
            client,
            url: webhook_url,
        }
    }

    pub fn listen(&self) {
        loop {
            let value = self.receiver.recv();
            let value = match value {
                Ok(v) => v,
                Err(e) => {
                    println!("{e}");
                    return;
                }
            };

            // let response = self.client.post(&self.url).json(&value).send().await;
            let response = self.client.post(&self.url).send_json(value);
            if let Err(e) = response {
                println!("{e}");
            }
        }
    }
}

/// Creates a DiscordLogger for the log create.
///
/// Set the DiscordLogger with log::set_boxed_logger()
///
/// You can put the DiscordLogListener in an Arc<Mutex<>> to move the
/// listener to a different thread.
pub fn discord_logger(webhook_url: String) -> (DiscordLogger, DiscordLogListener) {
    let (sender, receiver) = mpsc::channel::<Value>();

    let logger = DiscordLogger::new(sender);
    let listener = DiscordLogListener::new(receiver, webhook_url);

    (logger, listener)
}
