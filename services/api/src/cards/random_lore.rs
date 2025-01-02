use poem::error::InternalServerError;
use poem_openapi::payload::{Json, PlainText};
use sqlx::PgPool;
use wiki::wiki_client::WikiClient;

use crate::cache::{card_lore_data, RedisClient};

use super::lore::{GetCardLoreResponse, Lore};

pub async fn random_lore(
    pool: &PgPool,
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
) -> poem::Result<GetCardLoreResponse> {
    let card_ids: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT card_id
        FROM card
        ORDER BY random()
        LIMIT 30
    "#,
    )
    .fetch_all(pool)
    .await
    .map_err(InternalServerError)?;

    let mut lore: Option<Lore> = None;
    for card_id in card_ids {
        let wiki_lore = card_lore_data(redis_client, wiki_client, &card_id).await?;
        let current_lore = Lore::from(wiki_lore);
        if current_lore.has_a_translation() {
            lore = Some(current_lore);
            break;
        }
    }

    match lore {
        Some(lore) => Ok(GetCardLoreResponse::Lore(Json(lore))),
        None => Ok(GetCardLoreResponse::NotFound(PlainText(
            "Failed to get a card with lore! Try again later.".to_string(),
        ))),
    }
}
