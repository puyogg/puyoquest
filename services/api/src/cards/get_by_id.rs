use poem::{error::InternalServerError, web::Data, Result};
use poem_openapi::{
    param::Path,
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

pub async fn get_by_id(
    api_config: &ApiConfig,
    pool: &PgPool,
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    s3_client: &S3BackupClient,
    id: &str,
) -> Result<GetByIdResponse> {
    let card_db: Option<CardDb> = sqlx::query_as(
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
    .map_err(InternalServerError)?;

    match card_db {
        Some(card_db) => {
            let card = Card::upgrade_card_db(
                api_config,
                redis_client,
                wiki_client,
                s3_client,
                card_db
            ).await?;
            Ok(GetByIdResponse::Card(Json(card)))
        },
        None => Ok(GetByIdResponse::NotFound(PlainText(format!(
            "Character with id {} not found",
            &id,
        )))),
    }
}
