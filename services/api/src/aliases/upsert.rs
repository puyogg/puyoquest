use chrono::Utc;
use poem::error::InternalServerError;
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse,
};
use sqlx::PgPool;

use super::types::{Alias, AliasCreate};

#[derive(ApiResponse)]
pub enum UpsertResponse {
    #[oai(status = 200)]
    Ok(Json<Alias>),

    #[oai(status = 400)]
    InvalidName(PlainText<String>),
}

pub async fn upsert(pool: &PgPool, alias: &AliasCreate) -> poem::Result<UpsertResponse> {
    let normalized_alias = utils::normalize_name(&alias.alias);

    // TODO return bad request here if the normalized and trimmed alias is invalid
    if normalized_alias.len() == 0 {
        return Ok(UpsertResponse::InvalidName(PlainText(
            "Invalid alias".to_string(),
        )));
    }

    let updated_at = match &alias.updated_at {
        Some(d) => d,
        None => &Utc::now(),
    };
    let alias: Result<Alias, sqlx::Error> = sqlx::query_as(
        r#"
        INSERT INTO alias (alias, char_id, internal, card_type, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (alias)
        DO UPDATE SET
            char_id = EXCLUDED.char_id,
            internal = EXCLUDED.internal,
            card_type = EXCLUDED.card_type,
            updated_at = EXCLUDED.updated_at
        RETURNING *
    "#,
    )
    .bind(&normalized_alias)
    .bind(&alias.char_id)
    .bind(&alias.internal)
    .bind(&alias.card_type)
    .bind(&updated_at)
    .fetch_one(pool)
    .await;

    match alias {
        Ok(a) => Ok(UpsertResponse::Ok(Json(a))),
        Err(e) => match e {
            sqlx::Error::Database(db_error) => {
                println!("{}", &db_error);
                Err(InternalServerError(db_error))
            }
            _ => Err(InternalServerError(e)),
        },
    }
}
