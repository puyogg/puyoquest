use futures::future::try_join_all;
use poem::{Result, error::InternalServerError};
use poem_openapi::{ApiResponse, payload::Json};
use sqlx::PgPool;
use wiki::wiki_client::WikiClient;

use crate::{aws::s3::S3BackupClient, cache::RedisClient, config::ApiConfig};

use super::{Card, CardDb};

#[derive(ApiResponse)]
pub enum RandomCardsResponse {
    #[oai(status = 200)]
    Cards(Json<Vec<Card>>, #[oai(header = "Cache-Control")] String),
}

pub async fn random_cards(
    api_config: &ApiConfig,
    pool: &PgPool,
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    s3_client: &S3BackupClient,
    count: i32,
    exclude: Option<Vec<String>>,
) -> Result<RandomCardsResponse> {
    let exclude = exclude.unwrap_or(Vec::new());

    let card_dbs: Vec<CardDb> = sqlx::query_as(
        r#"
        SELECT *
        FROM card
        WHERE
            NOT (char_id = ANY($2))
        LIMIT $1
        "#,
    )
    .bind(&count)
    .bind(&exclude)
    .fetch_all(pool)
    .await
    .map_err(InternalServerError)?;

    let card_futures = card_dbs.into_iter().map(|card_db| {
        Card::upgrade_card_db(api_config, redis_client, wiki_client, s3_client, card_db)
    });
    let cards = try_join_all(card_futures).await?;

    Ok(RandomCardsResponse::Cards(
        Json(cards),
        "no-cache".to_string(),
    ))
}
