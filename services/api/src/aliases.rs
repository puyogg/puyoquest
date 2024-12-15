use crate::api_tag::ApiTag;

pub mod types;
use list::list_by_partial_name;
use poem::{web::Data, Result};
use poem_openapi::{
    param::{Path, Query},
    payload::{Json, PlainText},
    OpenApi,
};
use sqlx::PgPool;
use types::{Alias, AliasCreate};

pub mod find_by_alias;
pub use find_by_alias::{find_by_alias, query_find_by_alias, FindByAliasResponse};

pub mod list;
pub use list::{list_by_char_id, AliasListResponse};

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
    async fn list(
        &self,
        pool: Data<&PgPool>,
        char_id: Query<Option<String>>,
        name: Query<Option<String>>,
    ) -> Result<AliasListResponse> {
        let char_id = char_id.0;
        let name = name.0;

        match (char_id, name) {
            (Some(_char_id), Some(_name)) => Ok(AliasListResponse::BadRequest(PlainText(
                "Provide only one of char_id or name!".to_string(),
            ))),
            (Some(char_id), None) => list_by_char_id(pool.0, &char_id).await,
            (None, Some(name)) => list_by_partial_name(pool.0, &name).await,
            (None, None) => Ok(AliasListResponse::BadRequest(PlainText(
                "Missing char_id or name query parameter (only one)".to_string(),
            ))),
        }
    }

    #[oai(path = "/:name", method = "put")]
    async fn upsert(
        &self,
        pool: Data<&PgPool>,
        name: Path<String>,
        alias: Json<AliasCreate>,
    ) -> Result<UpsertResponse> {
        upsert(pool.0, &name.0, &alias.0).await
    }
}
