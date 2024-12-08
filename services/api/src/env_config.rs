use std::env;
use std::sync::LazyLock;

#[derive(Debug)]
pub enum DeploymentEnvironment {
    Local,
    Production,
}

#[derive(Debug)]
pub struct EnvConfig {
    pub environment: DeploymentEnvironment,
    pub pn_wiki_base_url: String,
    pub pn_wiki_api_url: String,
    pub redis_key_prefix: String,
    pub db_connection_string: Option<String>,
    pub redis_connection_string: Option<String>,
}

fn env_string(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(env::var(key)?)
}

fn load_env() -> EnvConfig {
    EnvConfig {
        environment: match env::var("ENVIRONMENT") {
            Err(_) => DeploymentEnvironment::Local,
            Ok(s) => match s.as_str() {
                "production" => DeploymentEnvironment::Production,
                "local" => DeploymentEnvironment::Local,
                _ => DeploymentEnvironment::Local,
            },
        },
        pn_wiki_base_url: env_string("PN_WIKI_BASE_URL")
            .unwrap_or("https://puyonexus.com/wiki".to_string()),
        pn_wiki_api_url: env_string("PN_WIKI_API_URL")
            .unwrap_or("https://puyonexus.com/mediawiki/api.php".to_string()),
        // redis_host: env_string("REDIS_HOST").unwrap_or("0.0.0.0".to_string()),
        // redis_port: env_string("REDIS_PORT").unwrap_or("36379".to_string()),
        // redis_uri_scheme: env_string("REDIS_URI_SCHEME").unwrap_or("redis".to_string()),
        redis_key_prefix: env_string("REDIS_KEY_PREFIX").unwrap_or("bot:".to_string()),
        db_connection_string: env::var("DB_CONNECTION_STRING").ok(),
        redis_connection_string: env::var("REDIS_CONNECTION_STRING").ok(),
    }
}

pub static ENV: LazyLock<EnvConfig> = LazyLock::new(|| load_env());
