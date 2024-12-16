use poem::{error::InternalServerError, Result};
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse,
};
use sqlx::PgPool;
use wiki::wiki_client::WikiClient;

use crate::{aws::s3::S3BackupClient, cache::RedisClient, config::ApiConfig};

use super::{Card, CardDb};

#[derive(ApiResponse)]
pub enum GetByIdResponse {
    #[oai(status = 200)]
    Card(Json<Card>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

pub async fn query_get_by_id(pool: &PgPool, id: &str) -> Result<Option<CardDb>, poem::Error> {
    sqlx::query_as(
        r#"
            SELECT *
            FROM card
            WHERE card_id = $1
            LIMIT 1
        "#,
    )
    .bind(&id)
    .fetch_optional(pool)
    .await
    .map_err(InternalServerError)
}

pub async fn fetch_card_by_id(
    api_config: &ApiConfig,
    pool: &PgPool,
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    s3_client: &S3BackupClient,
    id: &str,
) -> Result<Option<Card>, poem::Error> {
    let card_db = query_get_by_id(pool, id).await?;
    let card = match card_db {
        None => None,
        Some(card_db) => Some(
            Card::upgrade_card_db(api_config, redis_client, wiki_client, s3_client, card_db)
                .await?,
        ),
    };

    Ok(card)
}

pub async fn get_by_id(
    api_config: &ApiConfig,
    pool: &PgPool,
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    s3_client: &S3BackupClient,
    id: &str,
) -> Result<GetByIdResponse> {
    let card = fetch_card_by_id(api_config, pool, redis_client, wiki_client, s3_client, id).await?;

    match card {
        Some(c) => Ok(GetByIdResponse::Card(Json(c))),
        None => Ok(GetByIdResponse::NotFound(PlainText(format!(
            "Character with id {} not found",
            &id,
        )))),
    }
}
