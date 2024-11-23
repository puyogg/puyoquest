use poem::Result;
use poem_openapi::{
    payload::Json,
    ApiResponse, Object,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use wiki::wiki_client::WikiClient;

use crate::aws::s3::S3BackupClient;
use crate::cache;
use crate::cache::RedisClient;
use crate::cards::get_by_id::fetch_card_by_id;
use crate::cards::types::Card;
use crate::config::ApiConfig;

#[derive(Debug, Clone, Object, Serialize, Deserialize, PartialEq, Eq)]
pub struct CardsAndMaterials {
    pub cards: Vec<Card>,
    pub materials: Vec<Card>,
}

#[derive(ApiResponse)]
pub enum ListCardsResponse {
    #[oai(status = 200)]
    CardsAndMaterials(Json<CardsAndMaterials>),
}

pub async fn list_cards(
    api_config: &ApiConfig,
    pool: &PgPool,
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    s3_client: &S3BackupClient,
    char_id: &str,
    fetch_fresh: bool,
) -> Result<ListCardsResponse> {
    let character_and_material_ids =
        cache::character_card_ids(redis_client, wiki_client, char_id, fetch_fresh).await?;

    let card_futures = character_and_material_ids
        .card_ids
        .iter()
        .map(|card_id| {
            fetch_card_by_id(
                api_config,
                pool,
                redis_client,
                wiki_client,
                s3_client,
                card_id,
            )
        });
    let cards = futures::future::try_join_all(card_futures).await?
        .into_iter()
        .flatten()
        .collect();

    let material_futures = character_and_material_ids
        .material_ids
        .iter()
        .map(|card_id| {
            fetch_card_by_id(
                api_config,
                pool,
                redis_client,
                wiki_client,
                s3_client,
                card_id,
            )
        });
    let materials = futures::future::try_join_all(material_futures).await?
        .into_iter()
        .flatten()
        .collect();

    let result = CardsAndMaterials {
        cards,
        materials,
    };
    
    Ok(ListCardsResponse::CardsAndMaterials(Json(result)))
}
