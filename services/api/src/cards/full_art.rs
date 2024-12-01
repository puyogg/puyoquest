use poem_openapi::{
    Object,
    payload::{Json, PlainText},
    ApiResponse,
};
use sqlx::PgPool;
use wiki::wiki_client::WikiClient;
use serde::{Deserialize, Serialize};

use crate::{aws::s3::S3BackupClient, cache::RedisClient, config::ApiConfig};
use crate::cache;

use super::get_by_id::query_get_by_id;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Object)]
pub struct CardFullArtUrls {
    pub normal_left: Option<String>,
    pub normal_right: Option<String>,
    pub extra_power_left: Option<String>,
    pub extra_power_right: Option<String>,
    /// Full Power or Cross Ability
    pub ss: Option<String>,
    pub extra_power_ss: Option<String>,
    pub dual_shift_left: Option<String>,
    pub dual_shift_right: Option<String>,
    pub extra_power_dual_shift: Option<String>,
}

impl Default for CardFullArtUrls {
    fn default() -> Self {
        Self {
            normal_left: None,
            normal_right: None,
            extra_power_left: None,
            extra_power_right: None,
            ss: None,
            extra_power_ss: None,
            dual_shift_left: None,
            dual_shift_right: None,
            extra_power_dual_shift: None,
        }
    }
}

#[derive(ApiResponse)]
pub enum GetFullArtResponse {
    #[oai(status = 200)]
    FullArt(Json<CardFullArtUrls>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

pub async fn get_full_art(
    api_config: &ApiConfig,
    pool: &PgPool,
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    s3_client: &S3BackupClient,
    card_id: &str,
) -> poem::Result<GetFullArtResponse> {
    let card_db = query_get_by_id(pool, card_id).await?;
    if let None = card_db {
        return Ok(GetFullArtResponse::NotFound(PlainText(format!(
            "card_id not found: {}",
            card_id
        ))));
    }

    let card_db = match card_db {
        None => {
            return Ok(GetFullArtResponse::NotFound(PlainText(format!(
                "card_id not found: {}",
                card_id
            ))));
        },
        Some(c) => c,
    };

    let image_base_url = api_config.get_image_cache_domain().await?;
    let full_art = cache::card_art(
        redis_client,
        wiki_client,
        s3_client,
        &image_base_url,
        &card_db,
    ).await?;

    Ok(GetFullArtResponse::FullArt(Json(full_art)))
}
