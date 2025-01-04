use dashmap::{DashMap, DashSet};
use sdk::apis::configuration::Configuration;

pub mod card;
pub mod categorysearch;
pub mod char_by_id;
pub mod whoselore;

#[allow(unused)]
pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
pub type Context<'a> = poise::Context<'a, Data, Error>;

// Data which is stored and accessible in all command invocations
#[derive(Debug)]
pub struct Data {
    pub api_config: Configuration,
    /// Channel id for active lore games.
    pub active_lore_game: DashSet<String>,
    pub lore_score: DashMap<String, u64>,
}
