mod card_template_data;
pub use card_template_data::*;

mod character_series_data;
pub use character_series_data::*;

mod card_icons;
pub use card_icons::*;

mod character_card_ids;
pub use character_card_ids::*;

mod card_lore;
pub use card_lore::*;

mod card_art;
pub use card_art::*;

mod ppq_categories;
pub use ppq_categories::*;

use crate::{aws::ssm, env_config::DeploymentEnvironment};

#[derive(Debug)]
pub struct RedisClient {
    pub conn: redis::aio::MultiplexedConnection,
    prefix: String,
}

impl RedisClient {
    pub fn prefixed(&self, key: &str) -> String {
        format!("{}{key}", &self.prefix)
    }
}

pub async fn create_redis_connection(
    environment: &DeploymentEnvironment,
    redis_connection_string: &Option<String>,
    prefix: String,
) -> RedisClient {
    // let redis_conn_url = format!("{scheme}://:@{host}:{port}");
    match environment {
        DeploymentEnvironment::Local => {
            let redis_conn_url = match redis_connection_string {
                Some(u) => u,
                None => "redis://:@0.0.0.0:36379",
            };
            let conn = redis::Client::open(redis_conn_url)
                .expect("Invalid connection URL")
                .get_multiplexed_async_connection()
                .await
                .expect("failed to connect to redis");

            RedisClient { conn, prefix }
        },
        DeploymentEnvironment::Production => {
            let sdk_config = aws_config::from_env().load().await;
            let ssm_client = aws_sdk_ssm::Client::new(&sdk_config);

            let redis_conn_url = ssm::fetch_parameter(&ssm_client, "REDIS_CONNECTION_STRING").await.unwrap();
            let conn = redis::Client::open(redis_conn_url)
                .expect("Invalid connection URL")
                .get_multiplexed_async_connection()
                .await
                .expect("failed to connect to redis");

            RedisClient { conn, prefix }
        }
    }
}
