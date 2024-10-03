use crate::api_tag::ApiTag;

pub mod types;
use poem::{web::Data, Result};
use poem_openapi::{
    param::{Path, Query}, payload::Json, OpenApi
};
use sqlx::PgPool;
use types::{Alias, AliasCreate};

pub mod find_by_alias;
pub use find_by_alias::{find_by_alias, query_find_by_alias, FindByAliasResponse};

pub mod list_by_char_id;
pub use list_by_char_id::{list_by_char_id, ListByCharIdResponse};

pub mod upsert;
pub use upsert::{upsert, UpsertResponse};

pub struct AliasesRouter;

#[OpenApi(prefix_path = "/aliases", tag = "ApiTag::Aliases")]
impl AliasesRouter {
    /// Find an alias by name
    #[oai(path = "/:name", method = "get")]
    async fn find_by_alias(
        &self,
        pool: Data<&PgPool>,
        name: Path<String>,
    ) -> Result<FindByAliasResponse> {
        find_by_alias(pool.0, &name.0).await
    }

    /// List aliases for a char_id
    #[oai(path = "/", method = "get")]
    async fn list_by_char_id(
        &self,
        pool: Data<&PgPool>,
        char_id: Query<Option<String>>,
    ) -> Result<ListByCharIdResponse> {
        list_by_char_id(pool.0, &char_id.0).await
    }

    #[oai(path = "/:name", method = "put")]
    async fn upsert(&self, pool: Data<&PgPool>, name: Path<String>, alias: Json<AliasCreate>) -> Result<UpsertResponse> {
        upsert(pool.0, &name.0, &alias.0).await
    }
}
