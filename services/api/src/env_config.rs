use std::env;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct Env {
    pub pn_wiki_base_url: String,
    pub pn_wiki_api_url: String,
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
    }
}

pub static ENV: LazyLock<Env> = LazyLock::new(|| {
    load_env()
});
