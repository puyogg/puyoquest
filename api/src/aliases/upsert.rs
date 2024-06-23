use poem::error::InternalServerError;
use poem_openapi::{payload::Json, ApiResponse};
use sqlx::PgPool;

use crate::util::{normalize_name::normalize_name, url_encode_nfc::url_encode_nfc};

use super::types::{Alias, AliasCreate};

#[derive(ApiResponse)]
pub enum UpsertResponse {
    #[oai(status = 200)]
    Ok(Json<Alias>, #[oai(header = "Location")] String),
}

pub async fn upsert(
    pool: &PgPool,
    alias_name: &str,
    alias: &AliasCreate,
) -> poem::Result<UpsertResponse> {
    let normalized_alias = normalize_name(&alias_name);
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
    .bind(&alias.updated_at)
    .fetch_one(pool)
    .await;

    match alias {
        Ok(a) => {
            let name = a.alias.clone();
            let location = format!("/aliases/{}", url_encode_nfc(&name));
            Ok(UpsertResponse::Ok(Json(a), location))
        }
        Err(e) => match e {
            sqlx::Error::Database(db_error) => {
                println!("{}", &db_error);
                Err(InternalServerError(db_error))
            }
            _ => Err(InternalServerError(e)),
        },
    }
}
