use std::sync::Arc;

use crate::{
    aliases::query_find_by_alias,
    cache::RedisClient,
    util::{normalize_name::normalize_name, parse_rarity::parse_rarity},
};
use poem::{
    error::{FailedDependency, InternalServerError},
    http::StatusCode,
    Result,
};
use poem_openapi::{payload::Json, ApiResponse, Enum, Object};
use redis::{AsyncCommands, RedisError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use wiki::wiki_client::{FetchTemplate, WikiClient};

use crate::cards::types::Card;

use super::{template_data::CardTemplateData, types::CardDb};

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

pub async fn query_find_by_char_id_and_rarity(
    pool: &PgPool,
    char_id: &str,
    rarity: &str,
    rarity_modifier: &Option<String>,
) -> Result<Option<Card>> {
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

    let card = card.map(|c| Card::from(c));

    Ok(card)
}

pub async fn find_by_name_and_rarity(
    pool: &PgPool,
    wiki_client: &WikiClient,
    redis_client: &Arc<RedisClient>,
    name_query: &str,
    rarity_query: &str,
) -> Result<FindByNameAndRarityResponse> {
    let mut redis_conn = redis_client.conn.clone();

    let alias_name = normalize_name(name_query);

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

    // get wiki_template from redis cache
    let cached_wiki_template = redis_conn
        .get::<String, Option<String>>(
            redis_client.prefixed(format!("template:{}", &card.card_id).as_str()),
        )
        .await
        .inspect_err(|e| println!("{e}"))
        .map_err(InternalServerError)?
        .and_then(|w| {
            let list = serde_json::from_str::<Value>(&w);

            if let Err(e) = &list {
                println!("Error parsing cached wiki_template for: {}", &card.card_id);
                println!("{}", e);
            }

            list.ok()
        });

    let wiki_template = match cached_wiki_template {
        Some(c) => c.clone(),
        None => {
            let fetched_template = wiki_client
                .fetch_template(&card.card_id)
                .await
                .map_err(|e| FailedDependency(e))?;

            let key = redis_client.prefixed(format!("template:{}", &card.card_id).as_str());

            let _: std::result::Result<String, RedisError> =
                redis_conn.set(&key, &fetched_template.to_string()).await;
            let _ = redis_conn
                .expire::<&str, i64>(
                    &key, 604800, // 7 days
                )
                .await;

            fetched_template
        }
    };
    let wiki_template =
        serde_json::from_value::<CardTemplateData>(wiki_template).map_err(InternalServerError)?;

    let card_with_template = Card {
        wiki_template: Some(wiki_template),
        ..card
    };

    Ok(FindByNameAndRarityResponse::Card(Json(card_with_template)))
}
