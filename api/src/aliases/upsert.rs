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
    Ok(Json<Alias>, #[oai(header = "Location")] String),

    #[oai(status = 400)]
    MismatchedName(PlainText<String>),
}

pub async fn upsert(
    pool: &PgPool,
    alias_name: &str,
    alias: &AliasCreate,
) -> poem::Result<UpsertResponse> {
    if alias_name != &alias.alias {
        return Ok(UpsertResponse::MismatchedName(PlainText(format!(
            "Mismatched names: {}, {}",
            alias_name, &alias.alias
        ))));
    }

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
    .bind(&alias.alias)
    .bind(&alias.char_id)
    .bind(&alias.internal)
    .bind(&alias.card_type)
    .bind(&alias.updated_at)
    .fetch_one(pool)
    .await;

    match alias {
        Ok(a) => {
            let name = a.alias.clone();
            Ok(UpsertResponse::Ok(Json(a), format!("/aliases/{}", name)))
        }
        Err(e) => match e {
            sqlx::Error::Database(db_error) => {
                println!("{}", &db_error);
                Err(InternalServerError(db_error))
            },
            _ => Err(InternalServerError(e)),
        },
    }
}
