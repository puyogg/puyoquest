use std::env;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct Env {
    pub pn_wiki_base_url: String,
    pub pn_wiki_api_url: String,
    pub redis_host: String,
    pub redis_port: String,
    pub redis_key_prefix: String,
    /// redis or rediss for TLS
    pub redis_uri_scheme: String,
}

fn env_string(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(env::var(key)?)
}

fn load_env() -> Env {
    Env {
        pn_wiki_base_url: env_string("PN_WIKI_BASE_URL")
            .unwrap_or("https://puyonexus.com/wiki".to_string()),
        pn_wiki_api_url: env_string("PN_WIKI_API_URL")
            .unwrap_or("https://puyonexus.com/mediawiki/api.php".to_string()),
        redis_host: env_string("REDIS_HOST").unwrap_or("0.0.0.0".to_string()),
        redis_port: env_string("REDIS_PORT").unwrap_or("36379".to_string()),
        redis_key_prefix: env_string("REDIS_KEY_PREFIX").unwrap_or("bot:".to_string()),
        redis_uri_scheme: env_string("REDIS_URI_SCHEME").unwrap_or("redis".to_string()),
    }
}

pub static ENV: LazyLock<Env> = LazyLock::new(|| load_env());
