use std::env;

use anyhow::anyhow;

#[derive(Debug)]
pub enum DeploymentEnvironment {
    Local,
    Production,
}

#[derive(Debug)]
pub struct EnvConfig {
    pub environment: DeploymentEnvironment,
    pub db_connection_string: String,
    pub ppq_api_base_url: String,
    pub webhook_url: String,
    pub default_days_ago: i64,
    pub owner_id: String,
}

impl EnvConfig {
    pub async fn new() -> Result<EnvConfig, anyhow::Error> {
        let environment = match env::var("ENVIRONMENT") {
            Err(_) => DeploymentEnvironment::Local,
            Ok(s) => match s.as_str() {
                "production" => DeploymentEnvironment::Production,
                "local" => DeploymentEnvironment::Local,
                _ => DeploymentEnvironment::Local,
            },
        };

        let sdk_config = aws_config::from_env().load().await;
        let ssm_client = aws_sdk_ssm::Client::new(&sdk_config);

        let load_ssm_or_env = |key: &'static str, default: &'static str| {
            load_ssm_or_env(&environment, &ssm_client, key, default)
        };

        let db_connection_string = load_ssm_or_env(
            "PPQ_DB_CONNECTION_STRING",
            "postgres://postgres:password@localhost:35432/ppq_api_db",
        )
        .await?;

        let owner_id = env::var("OWNER_ID")?;

        Ok(EnvConfig {
            environment,
            db_connection_string,
            ppq_api_base_url: env::var("PPQ_API_BASE_URL")
                .unwrap_or("http://localhost:3000".to_string()),
            webhook_url: env::var("WEBHOOK_URL")?,
            default_days_ago: env::var("DEFAULT_DAYS_AGO")?.parse::<i64>().unwrap_or(7),
            owner_id,
        })
    }
}

async fn fetch_parameter(
    ssm_client: &aws_sdk_ssm::Client,
    key: &str,
) -> Result<String, anyhow::Error> {
    ssm_client
        .get_parameter()
        .name(key)
        .with_decryption(true)
        .send()
        .await?
        .parameter
        .map(|p| p.value)
        .flatten()
        .ok_or(anyhow!("Failed to load ssm parameter: {}", key))
}

async fn load_ssm_or_env(
    environment: &DeploymentEnvironment,
    ssm_client: &aws_sdk_ssm::Client,
    key: &str,
    local_default: &str,
) -> Result<String, anyhow::Error> {
    match environment {
        DeploymentEnvironment::Local => {
            let env_value = std::env::var(key);

            match env_value {
                Ok(v) => Ok(v),
                Err(_) => Ok(local_default.to_string()),
            }
        }
        DeploymentEnvironment::Production => {
            let env_value = std::env::var(key);
            match env_value {
                Ok(v) => return Ok(v),
                Err(_) => (),
            };

            let ssm_value = fetch_parameter(&ssm_client, key).await?;
            Ok(ssm_value)
        }
    }
}
