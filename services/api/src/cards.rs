use std::sync::Arc;

use crate::{api_tag::ApiTag, aws::s3::S3BackupClient, cache::RedisClient, config::ApiConfig};
use poem::web::Data;
use poem_openapi::{
    param::{Path, Query},
    payload::Json,
    OpenApi,
};
use sqlx::PgPool;

pub mod template_data;

pub mod types;
use types::{Card, CardCreate, CardDb};

pub mod get_by_id;
use get_by_id::{get_by_id, GetByIdResponse};

pub mod upsert;
use upsert::{upsert, UpsertResponse};

pub mod find_by_name_rarity;
use find_by_name_rarity::{find_by_name_and_rarity, FindByNameAndRarityResponse};

pub mod lore;
use lore::{GetCardLoreResponse, get_card_lore};

pub mod full_art;
use full_art::{get_full_art, GetFullArtResponse};

pub struct CardsRouter;

#[OpenApi(prefix_path = "/cards", tag = "ApiTag::Cards")]
impl CardsRouter {
    /// Find by card_id
    #[oai(path = "/:id", method = "get")]
    async fn get(
        &self,
        api_config: Data<&Arc<ApiConfig>>,
        pool: Data<&PgPool>,
        redis_client: Data<&Arc<RedisClient>>,
        wiki_client: Data<&wiki::wiki_client::WikiClient>,
        s3_client: Data<&Arc<S3BackupClient>>,
        id: Path<String>,
    ) -> poem::Result<GetByIdResponse> {
        get_by_id(
            api_config.0,
            pool.0,
            redis_client.0,
            wiki_client.0,
            s3_client.0,
            &id.0,
        )
        .await
    }

    /// Find by name and rarity
    #[oai(path = "/", method = "get")]
    async fn find(
        &self,
        api_config: Data<&Arc<ApiConfig>>,
        pool: Data<&PgPool>,
        wiki_client: Data<&wiki::wiki_client::WikiClient>,
        redis_client: Data<&Arc<RedisClient>>,
        s3_client: Data<&Arc<S3BackupClient>>,
        name: Query<Option<String>>,
        rarity: Query<Option<String>>,
    ) -> poem::Result<FindByNameAndRarityResponse> {
        let name = name.0;
        let rarity = rarity.0;
        let redis_client = redis_client.0.clone();

        if let (Some(n), Some(r)) = (&name, &rarity) {
            return find_by_name_and_rarity(
                api_config.0,
                pool.0,
                wiki_client.0,
                &redis_client,
                s3_client.0,
                n,
                r,
            )
            .await;
        }

        let mut missing_params: Vec<String> = Vec::new();
        if name.is_none() {
            missing_params.push("name".to_string());
        }
        if rarity.is_none() {
            missing_params.push("rarity".to_string());
        }
        Ok(FindByNameAndRarityResponse::MissingNameOrRarity(Json(
            find_by_name_rarity::BadRequestReason::missing_params(missing_params),
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

    /// Get card lore
    #[oai(path = "/:card_id/lore", method = "get")]
    async fn get_lore(
        &self,
        pool: Data<&PgPool>,
        redis_client: Data<&Arc<RedisClient>>,
        wiki_client: Data<&wiki::wiki_client::WikiClient>,
        card_id: Path<String>,
    ) -> poem::Result<GetCardLoreResponse> {
        get_card_lore(
            &pool.0,
            &redis_client.0, 
            &wiki_client.0,
            &card_id.0,
        ).await
    }

    /// Get card full art (all orientations)
    #[oai(path = "/:card_id/full-art", method = "get")]
    async fn get_full_art(
        &self,
        api_config: Data<&Arc<ApiConfig>>,
        pool: Data<&PgPool>,
        redis_client: Data<&Arc<RedisClient>>,
        wiki_client: Data<&wiki::wiki_client::WikiClient>,
        s3_client: Data<&Arc<S3BackupClient>>,
        card_id: Path<String>,
    ) -> poem::Result<GetFullArtResponse> {
        get_full_art(
            api_config.0,
            pool.0,
            redis_client.0,
            wiki_client.0,
            s3_client.0,
            &card_id.0,
        ).await
    }

    // /// List random cards
    // /// TODO: Set cache control
    // #[oai(path = "/random", method = "get")]
    // async fn random(&self) -> Result<()> {
    //     todo!();
    // }
}
