use std::sync::Arc;

use crate::{api_tag::ApiTag, cache::RedisClient};
use poem::web::Data;
use poem_openapi::{
    param::{Path, Query},
    payload::Json,
    OpenApi,
};
use sqlx::PgPool;

pub mod template_data;

pub mod types;
use types::{Card, CardCreate};

pub mod get_by_id;
use get_by_id::{get_by_id, GetByIdResponse};

pub mod upsert;
use upsert::{upsert, UpsertResponse};

pub mod find_by_name_rarity;
use find_by_name_rarity::{find_by_name_and_rarity, FindByNameAndRarityResponse};

pub struct CardsRouter;

#[OpenApi(prefix_path = "/cards", tag = "ApiTag::Cards")]
impl CardsRouter {
    /// Find by card_id
    #[oai(path = "/:id", method = "get")]
    async fn get(&self, pool: Data<&PgPool>, id: Path<String>) -> poem::Result<GetByIdResponse> {
        get_by_id(pool, id).await
    }

    /// Find by name and rarity
    /// OR find by category
    #[oai(path = "/", method = "get")]
    async fn find(
        &self,
        pool: Data<&PgPool>,
        wiki_client: Data<&wiki::wiki_client::WikiClient>,
        redis_client: Data<&Arc<RedisClient>>,
        name: Query<Option<String>>,
        rarity: Query<Option<String>>,
    ) -> poem::Result<FindByNameAndRarityResponse> {
        let name = name.0;
        let rarity = rarity.0;
        let redis_client = redis_client.0.clone();

        if let (Some(n), Some(r)) = (&name, &rarity) {
            return find_by_name_and_rarity(pool.0, wiki_client.0, &redis_client, n, r).await
        }

        let mut missing_params: Vec<String> = Vec::new();
        if name.is_none() {
            missing_params.push("name".to_string());
        }
        if rarity.is_none() {
            missing_params.push("rarity".to_string());
        }
        Ok(FindByNameAndRarityResponse::MissingNameOrRarity(Json(
            find_by_name_rarity::BadRequestReason::missing_params(missing_params)
        )))
    }

    /// Upsert card data (admins only)
    #[oai(path = "/", method = "post")]
    async fn upsert(
        &self,
        pool: Data<&PgPool>,
        card: Json<CardCreate>,
    ) -> poem::Result<UpsertResponse> {
        // TODO: Apply NFKD normalization before saving
        upsert(pool.0, &card.0).await
    }

    // /// List random cards
    // /// TODO: Set cache control
    // #[oai(path = "/random", method = "get")]
    // async fn random(&self) -> Result<()> {
    //     todo!();
    // }
}
