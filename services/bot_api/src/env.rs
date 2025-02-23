use poem::error::InternalServerError;
use std::env::VarError;
use thiserror::Error;

#[derive(Debug)]
pub enum DeploymentEnvironment {
    Local,
    Production,
}

#[derive(Debug)]
pub struct BotApiEnv {
    pub environment: DeploymentEnvironment,
    pub db_connection_string: Option<String>,
}

#[derive(Debug)]
struct ParameterStoreFetchError {}
impl std::error::Error for ParameterStoreFetchError {}
impl std::fmt::Display for ParameterStoreFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to fetch value from parameter store.")
    }
}

async fn fetch_parameter(
    ssm_client: &aws_sdk_ssm::Client,
    name: &str,
) -> Result<String, poem::Error> {
    ssm_client
        .get_parameter()
        .name(name)
        .with_decryption(true)
        .send()
        .await
        .map_err(InternalServerError)
        .map(|p| p.parameter)?
        .and_then(|p| p.value)
        .ok_or(ParameterStoreFetchError {})
        .map_err(InternalServerError)
}

async fn env_or_default(
    deployment_env: DeploymentEnvironment,
    ssm_client: &aws_sdk_ssm::Client,
    key: &str,
    default: &str,
) -> Result<String, poem::Error> {
    match deployment_env {
        DeploymentEnvironment::Local => {
            let var = std::env::var(key).ok();
            match var {
                Some(v) => Ok(v),
                None => {
                    tracing::info!("Missing env var for {}. Defaulting to: {}", key, default);
                    Ok(default.to_string())
                }
            }
        }
        DeploymentEnvironment::Production => fetch_parameter(ssm_client, key)
            .await
            .inspect_err(|e| {
                tracing::error!("Failed to load parameter: {}. {:?}", key, e);
            })
            .map_err(InternalServerError),
    }
}

pub async fn load_env() -> BotApiEnv {
    let deployment_env = match std::env::var("ENVIRONMENT")
        .unwrap_or("local".to_string())
        .as_str()
    {
        "production" => DeploymentEnvironment::Production,
        "local" => DeploymentEnvironment::Local,
        _ => {
            tracing::warn!("ENVIRONMENT not set. Defaulting to \"local\"");
            DeploymentEnvironment::Local
        }
    };

    todo!();
}
