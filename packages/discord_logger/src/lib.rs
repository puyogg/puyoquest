use std::sync::mpsc::{self, Receiver, Sender};
use std::{str::FromStr, sync::LazyLock};

use log::Level;
use serde_json::{json, Value};
use std::time::Duration;
use ureq::Agent;

static RUST_LOG: LazyLock<Option<String>> = LazyLock::new(|| std::env::var("RUST_LOG").ok());

pub struct DiscordLogger {
    sender: Sender<Value>,
}

impl DiscordLogger {
    pub fn new(sender: Sender<Value>) -> DiscordLogger {
        DiscordLogger { sender }
    }
}

impl log::Log for DiscordLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let log_level = RUST_LOG
            .as_ref()
            .map(|l| Level::from_str(&l).ok())
            .flatten()
            .unwrap_or(Level::Info);

        metadata.level() <= log_level
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

pub fn discord_logger(webhook_url: String) -> (DiscordLogger, DiscordLogListener) {
    let (sender, receiver) = mpsc::channel::<Value>();

    let logger = DiscordLogger::new(sender);
    let listener = DiscordLogListener::new(receiver, webhook_url);

    (logger, listener)
}
