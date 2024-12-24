use crate::api_tag::ApiTag;

pub mod types;
use list::{list_by_exact_name, list_by_partial_name};
use poem::{error::BadRequest, web::Data, Result};
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    OpenApi,
};
use sqlx::PgPool;
use types::{Alias, AliasCreate};

pub mod list;
pub use list::{list_by_char_id, AliasListResponse};

pub mod upsert;
pub use upsert::{upsert, UpsertResponse};

mod delete;
use delete::{delete, AliasDeleteResponse};

pub struct AliasesRouter;

#[OpenApi(prefix_path = "/aliases", tag = "ApiTag::Aliases")]
impl AliasesRouter {
    /// List aliases for a char_id
    #[oai(path = "/", method = "get")]
    async fn list(
        &self,
        pool: Data<&PgPool>,
        char_id: Query<Option<String>>,
        name: Query<Option<String>>,
        exact: Query<Option<String>>,
    ) -> Result<AliasListResponse> {
        let char_id = char_id.0;
        let name = name.0;
        let exact = exact.0;
        let exact = match exact {
            Some(e) => match e.as_ref() {
                "true" => true,
                _ => false,
            },
            None => false,
        };

        match (char_id, name) {
            (Some(_char_id), Some(_name)) => Ok(AliasListResponse::BadRequest(PlainText(
                "Provide only one of char_id or name!".to_string(),
            ))),
            (Some(char_id), None) => list_by_char_id(pool.0, &char_id).await,
            (None, Some(name)) => match exact {
                true => list_by_exact_name(pool.0, &name).await,
                false => list_by_partial_name(pool.0, &name).await,
            },
            (None, None) => Ok(AliasListResponse::BadRequest(PlainText(
                "Missing char_id or name query parameter (only one)".to_string(),
            ))),
        }
    }

    #[oai(path = "/", method = "post")]
    async fn upsert(
        &self,
        pool: Data<&PgPool>,
        alias: Json<AliasCreate>,
    ) -> Result<UpsertResponse> {
        upsert(pool.0, &alias.0).await
    }

    #[oai(path = "/", method = "delete")]
    async fn delete(
        &self,
        pool: Data<&PgPool>,
        name: Query<String>,
    ) -> Result<AliasDeleteResponse> {
        delete(pool.0, &name.0).await
    }
}
