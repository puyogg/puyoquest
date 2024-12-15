use std::collections::HashSet;

use futures::try_join;
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

    let ilike_startswith = format!("{}%", &normalized_name);
    let ilike_any = format!("%{}%", &normalized_name);

    let (alias_startswith, alias_any, alias_leven) = try_join!(
        query(pool, &ilike_startswith),
        query(pool, &ilike_any),
        query_leven(pool, &normalized_name),
    )?;

    let mut results: Vec<Alias> = Vec::with_capacity(9);
    let mut unique_aliases: HashSet<String> = HashSet::new();
    for (max, aliases) in [(4, &alias_startswith), (3, &alias_any), (1, &alias_leven)] {
        let mut count = 0;
        for a in aliases {
            if count == max {
                break;
            }

            if unique_aliases.contains(&a.alias) {
                continue;
            }

            count += 1;
            unique_aliases.insert(a.alias.clone());
            results.push(a.clone());
        }
    }

    Ok(AliasListResponse::Aliases(Json(results)))
}

async fn query(pool: &PgPool, ilike: &str) -> Result<Vec<Alias>> {
    sqlx::query_as(
        r#"
        SELECT *
        FROM alias
        WHERE internal = TRUE
            AND alias ILIKE $1
        ORDER BY LENGTH(alias)
        LIMIT 5
    "#,
    )
    .bind(ilike)
    .fetch_all(pool)
    .await
    .map_err(InternalServerError)
}

async fn query_leven(pool: &PgPool, name: &str) -> Result<Vec<Alias>> {
    sqlx::query_as(
        r#"
        SELECT *
        FROM alias
        WHERE internal = TRUE
        ORDER BY levenshtein(alias, $1)
        LIMIT 5
    "#,
    )
    .bind(name)
    .fetch_all(pool)
    .await
    .map_err(InternalServerError)
}
