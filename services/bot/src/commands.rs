use dashmap::{DashMap, DashSet};
use sdk::apis::configuration::Configuration;

use crate::aws::AwsClient;

pub mod card;
pub mod categorysearch;
pub mod char_by_id;
pub mod incorrect_quote;
pub mod leaderboard;
pub mod ntc;
pub mod ppq_events;
pub mod reindex;
pub mod say_in;
pub mod server_settings;
pub mod whoselore;
pub mod pin;

#[allow(unused)]
pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
pub type Context<'a> = poise::Context<'a, Data, Error>;

// Data which is stored and accessible in all command invocations
#[derive(Debug)]
pub struct Data {
    pub api_config: Configuration,
    pub aws_client: AwsClient,
    /// Channel id for active lore games.
    pub active_lore_game: DashSet<String>,
    pub lore_score: DashMap<String, u64>,
    pub bot_api_config: bot_sdk::apis::configuration::Configuration,
}
