use poem::{error::InternalServerError, Result};
use poem_openapi::{payload::{Json, PlainText}, ApiResponse};
use sqlx::PgPool;

use crate::util::normalize_name::normalize_name;

use super::types::Character;

#[derive(ApiResponse)]
pub enum FindResponse {
    #[oai(status = 200)]
    Characters(Json<Vec<Character>>),

    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

async fn find_by_alias(pool: &PgPool, alias_name: &str) -> Result<FindResponse> {
    let alias_name = normalize_name(alias_name);

    let character = sqlx::query_as::<_, Character>(
        r#"
            SELECT character.*
            FROM alias
            LEFT JOIN character ON character.char_id = alias.char_id
            WHERE alias.alias = $1
            LIMIT 1
        "#,
    )
    .bind(&alias_name)
    .fetch_optional(pool)
    .await
    .map_err(InternalServerError)?;

    match character {
        Some(c) => {
            let characters: Vec<Character> = vec![c];
            Ok(FindResponse::Characters(Json(characters)))
        }
        None => {
            let characters: Vec<Character> = vec![];
            Ok(FindResponse::Characters(Json(characters)))
        }
    }
}

pub async fn find(pool: &PgPool, alias_name: Option<&str>) -> Result<FindResponse> {
    if alias_name.is_some() {
        let response = find_by_alias(pool, &alias_name.unwrap()).await?;
        return Ok(response);
    }

    Ok(FindResponse::BadRequest(PlainText("Missing query parameter: alias".to_string())))
}
