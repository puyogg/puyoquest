use crate::aliases::types::Alias;
use crate::{
    aws::s3::S3BackupClient, cache::RedisClient, config::ApiConfig,
    util::parse_rarity::parse_rarity,
};
use poem::{error::InternalServerError, http::StatusCode, Result};
use poem_openapi::{payload::Json, ApiResponse, Enum, Object};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use wiki::wiki_client::WikiClient;

use crate::cards::types::Card;

use super::types::CardDb;

#[derive(Enum, Clone, Debug, Serialize, Deserialize)]
#[oai(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum NotFoundReasonEnum {
    Alias,
    Rarity,
}

#[derive(Debug, Clone, Object, Serialize)]
pub struct NotFoundReason {
    pub not_found: NotFoundReasonEnum,
}

#[derive(Debug, Clone, Object, Serialize)]
pub struct BadRequestReason {
    title: String,
    status: u16,
    detail: String,
    #[serde(rename(serialize = "invalid-params"))]
    invalid_params: Vec<String>,
}

impl BadRequestReason {
    pub fn missing_params(missing: Vec<String>) -> BadRequestReason {
        BadRequestReason {
            status: StatusCode::BAD_REQUEST.as_u16(),
            title: String::from("Missing name and/or rarity"),
            detail: String::from("name and rarity are both required query parameters."),
            invalid_params: missing,
        }
    }
}

#[derive(ApiResponse)]
pub enum FindByNameAndRarityResponse {
    #[oai(status = 200)]
    Card(Json<Card>),

    #[oai(status = 400)]
    MissingNameOrRarity(Json<BadRequestReason>),

    #[oai(status = 404)]
    NotFound(Json<NotFoundReason>),
}

pub async fn query_find_by_alias(pool: &PgPool, alias_name: &str) -> Result<Option<Alias>> {
    let alias: Option<Alias> = sqlx::query_as(
        r#"
            SELECT *
            FROM alias
            WHERE alias = $1
            LIMIT 1
        "#,
    )
    .bind(&alias_name)
    .fetch_optional(pool)
    .await
    .map_err(InternalServerError)?;

    Ok(alias)
}

pub async fn query_find_by_char_id_and_rarity(
    pool: &PgPool,
    char_id: &str,
    rarity: &str,
    rarity_modifier: &Option<String>,
) -> Result<Option<CardDb>> {
    let card: Option<CardDb> = match rarity_modifier {
        Some(modifier) => sqlx::query_as(
            r#"
                    SELECT *
                    FROM card
                    WHERE char_id = $1
                        AND rarity = $2
                        AND rarity_modifier = $3
                    LIMIT 1
                "#,
        )
        .bind(&char_id)
        .bind(&rarity)
        .bind(&modifier)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError),
        None => sqlx::query_as(
            r#"
                    SELECT *
                    FROM card
                    WHERE char_id = $1
                        AND rarity = $2
                    LIMIT 1
                "#,
        )
        .bind(&char_id)
        .bind(&rarity)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError),
    }?;

    Ok(card)
}

pub async fn find_by_name_and_rarity(
    api_config: &ApiConfig,
    pool: &PgPool,
    wiki_client: &WikiClient,
    redis_client: &RedisClient,
    s3_client: &S3BackupClient,
    name_query: &str,
    rarity_query: &str,
) -> Result<FindByNameAndRarityResponse> {
    let alias_name = utils::normalize_name(name_query);

    let alias_lookup = query_find_by_alias(pool, &alias_name).await?;

    let alias = match alias_lookup {
        Some(a) => a,
        None => {
            return Ok(FindByNameAndRarityResponse::NotFound(Json(
                NotFoundReason {
                    not_found: NotFoundReasonEnum::Alias,
                },
            )))
        }
    };

    let rarity_and_modifier = match parse_rarity(rarity_query) {
        Ok(r) => r,
        Err(_) => {
            return Ok(FindByNameAndRarityResponse::NotFound(Json(
                NotFoundReason {
                    not_found: NotFoundReasonEnum::Rarity,
                },
            )))
        }
    };

    let card = query_find_by_char_id_and_rarity(
        pool,
        &alias.char_id,
        &rarity_and_modifier.rarity,
        &rarity_and_modifier.modifier,
    )
    .await?;

    let card = match card {
        Some(c) => c,
        None => {
            return Ok(FindByNameAndRarityResponse::NotFound(Json(
                NotFoundReason {
                    not_found: NotFoundReasonEnum::Rarity,
                },
            )))
        }
    };

    let card_with_extras =
        Card::upgrade_card_db(api_config, redis_client, wiki_client, s3_client, card).await?;

    Ok(FindByNameAndRarityResponse::Card(Json(card_with_extras)))
}
