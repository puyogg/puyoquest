use moka::future::Cache;
use std::env;
use std::time::Duration;

use crate::aws::ssm::fetch_parameter;

pub struct ApiConfig {
    cache: Cache<String, String>,
    ssm_client: aws_sdk_ssm::Client,
}

const IMAGE_CACHE_KEY: &'static str = "IMAGE_CACHE_BASE_URL";

impl ApiConfig {
    pub async fn new() -> Result<ApiConfig, poem::Error> {
        let cache = Cache::builder()
            .max_capacity(10)
            .time_to_live(Duration::from_secs(3600))
            .build();

        let sdk_config = aws_config::from_env().load().await;
        let ssm_client = aws_sdk_ssm::Client::new(&sdk_config);

        let image_cache_domain = match env::var(IMAGE_CACHE_KEY) {
            Ok(s) => s,
            Err(_) => fetch_parameter(&ssm_client, IMAGE_CACHE_KEY).await?,
        };
        cache
            .insert(IMAGE_CACHE_KEY.to_string(), image_cache_domain.clone())
            .await;

        Ok(ApiConfig { cache, ssm_client })
    }

    pub async fn get_image_cache_domain(&self) -> Result<String, poem::Error> {
        let cached_domain = self.cache.get(IMAGE_CACHE_KEY).await;

        match cached_domain {
            None => {
                let domain = fetch_parameter(&self.ssm_client, IMAGE_CACHE_KEY).await?;
                self.cache
                    .insert(IMAGE_CACHE_KEY.to_string(), domain.clone())
                    .await;
                Ok(domain)
            }
            Some(c) => Ok(c),
        }
    }
}
