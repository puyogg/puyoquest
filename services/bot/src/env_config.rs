use std::{cell::OnceCell, env};
use std::sync::LazyLock;

#[derive(Debug)]
pub struct Env {
    pub bot_token: String,
    pub primary_server_id: u64,
    pub client_id: u64,
    pub db_url: String,
    pub db_host: String,
    pub db_name: String,
    pub ppq_api_host: String,
}

fn env_string(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(env::var(key).expect(&format!("Missing {}", key)))
}

fn env_u64(key: impl AsRef<str>) -> Result<u64, Box<dyn std::error::Error>> {
    let key = key.as_ref();
    let result = env::var(key)
        .expect(&format!("Missing {}", key))
        .parse::<u64>()
        .expect(&format!("Invalid {}! Must be a u64", key));

    Ok(result)
}

fn load_env() -> Env {
    Env {
        bot_token: env_string("BOT_TOKEN").unwrap(),
        primary_server_id: env_u64("PRIMARY_SERVER_ID").unwrap(),
        client_id: env_u64("CLIENT_ID").unwrap(),
        db_url: env_string("DB_URL").unwrap(),
        db_host: env_string("DB_HOST").unwrap(),
        db_name: env_string("DB_NAME").unwrap(),
        ppq_api_host: env_string("PPQ_API_HOST").unwrap(),
    }
}

pub fn load_env_file() -> Result<Env, Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    Ok(load_env())
}

pub static ENV: LazyLock<Env> = LazyLock::new(|| {
    load_env_file().unwrap()
});
