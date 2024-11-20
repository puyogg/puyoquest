use std::sync::Arc;

use crate::aliases;
use crate::api_tag::ApiTag;
use crate::aws::s3::S3BackupClient;
use crate::cache::RedisClient;
use crate::config::ApiConfig;
use poem::{web::Data, Result};
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, OpenApi};
use sqlx::PgPool;

pub mod get_by_id;
use get_by_id::get_by_id;
use get_by_id::GetByIdResponse;

pub mod upsert;
pub use upsert::{upsert, UpsertResponse};

pub mod find;
pub use find::{find, FindResponse};

pub mod types;
use types::{CharacterCreate, CharacterDb};

pub mod list_cards;

pub struct CharactersRoute;

#[OpenApi(prefix_path = "/characters", tag = "ApiTag::Characters")]
impl CharactersRoute {
    /// TODO: Option to refresh index
    #[oai(path = "/:id", method = "get")]
    async fn get_by_id(&self, pool: Data<&PgPool>, id: Path<String>) -> Result<GetByIdResponse> {
        get_by_id(pool.0, &id.0).await
    }

    /// Create a character or update one if it already exists
    #[oai(path = "/:id", method = "put")]
    async fn upsert(
        &self,
        pool: Data<&PgPool>,
        id: Path<String>,
        character: Json<CharacterCreate>,
    ) -> Result<UpsertResponse> {
        // TODO: Only allow admins to upsert characters
        upsert(pool.0, &id.0, &character.0).await
    }

    #[oai(path = "/:id/aliases", method = "get")]
    async fn list_aliases(
        &self,
        pool: Data<&PgPool>,
        id: Path<String>,
    ) -> Result<aliases::list_by_char_id::ListByCharIdResponse> {
        aliases::list_by_char_id(pool.0, &Some(id.0)).await
    }

    /// Find by alias or category
    #[oai(path = "/", method = "get")]
    async fn find(
        &self,
        pool: Data<&PgPool>,
        alias: Query<Option<String>>,
    ) -> Result<FindResponse> {
        let alias_name: Option<&str> = alias.0.as_deref();
        find(pool.0, alias_name).await
    }

    /// List a character's cards
    #[oai(path = "/:id/cards", method = "get")]
    async fn list_cards(
        &self,
        api_config: Data<&Arc<ApiConfig>>,
        pool: Data<&PgPool>,
        redis_client: Data<&Arc<RedisClient>>,
        wiki_client: Data<&wiki::wiki_client::WikiClient>,
        s3_client: Data<&Arc<S3BackupClient>>,
        id: Path<String>,
    ) -> Result<()> {
        Ok(())
    }
}
