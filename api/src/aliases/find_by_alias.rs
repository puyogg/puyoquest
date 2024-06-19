use poem::{error::InternalServerError, Result};
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse,
};
use sqlx::PgPool;

use super::Alias;

#[derive(ApiResponse)]
pub enum FindByAliasResponse {
    #[oai(status = 200)]
    Alias(Json<Alias>),

    #[oai(status = 400)]
    BadRequest(PlainText<String>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

pub async fn find_by_alias(pool: &PgPool, alias_name: &str) -> Result<FindByAliasResponse> {
    // TODO: normalize alias_name before looking it up in the database

    let alias: Option<Alias> = sqlx::query_as(
        r#"
            SELECT *
            FROM alias
            WHERE alias = $1
        "#,
    )
    .bind(&alias_name)
    .fetch_optional(pool)
    .await
    .map_err(InternalServerError)?;

    match alias {
        Some(a) => Ok(FindByAliasResponse::Alias(Json(a))),
        None => Ok(FindByAliasResponse::NotFound(PlainText(format!(
            "Alias {} not found",
            &alias_name,
        )))),
    }
}
