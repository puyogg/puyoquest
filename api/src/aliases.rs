use crate::api_tag::ApiTag;

pub mod types;
use find_by_alias::FindByAliasResponse;
use poem::{web::Data, Result};
use poem_openapi::{
    param::{Path, Query},
    OpenApi,
};
use sqlx::PgPool;
use types::Alias;

pub mod find_by_alias;
pub use find_by_alias::find_by_alias;

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
        find_by_alias::find_by_alias(pool.0, &name.0).await
    }

    /// List aliases for a char_id
    #[oai(path = "/", method = "get")]
    async fn list_by_char_id(
        &self,
        pool: Data<&PgPool>,
        name: Query<Option<String>>,
    ) -> Result<()> {
        todo!()
    }
}
