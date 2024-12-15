use poem::{error::InternalServerError, Result};
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse,
};
use sqlx::PgPool;

use crate::util::normalize_name::normalize_name;

use super::Alias;

#[derive(ApiResponse)]
pub enum AliasListResponse {
    #[oai(status = 200)]
    Aliases(Json<Vec<Alias>>),

    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

pub async fn list_by_char_id(pool: &PgPool, char_id: &str) -> Result<AliasListResponse> {
    let aliases: Vec<Alias> = sqlx::query_as(
        r#"
        SELECT *
        FROM alias
        WHERE char_id = $1
    "#,
    )
    .bind(char_id)
    .fetch_all(pool)
    .await
    .map_err(InternalServerError)?;

    Ok(AliasListResponse::Aliases(Json(aliases)))
}

pub async fn list_by_partial_name(pool: &PgPool, name: &str) -> Result<AliasListResponse> {
    let normalized_name = normalize_name(name);

    let aliases: Vec<Alias> = sqlx::query_as(
        r#"
        SELECT *
        FROM alias
        WHERE internal = TRUE
        ORDER BY levenshtein(alias, $1)
        LIMIT 5
    "#,
    )
    .bind(&normalized_name)
    .fetch_all(pool)
    .await
    .map_err(InternalServerError)?;

    Ok(AliasListResponse::Aliases(Json(aliases)))
}
