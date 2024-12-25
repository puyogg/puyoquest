use poem::error::InternalServerError;
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse, Object,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::types::Alias;

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct DeleteCount {
    count: u64,
}

#[derive(ApiResponse)]
pub enum AliasDeleteResponse {
    #[oai(status = 200)]
    SuccessfullyDeleted(Json<DeleteCount>),

    #[oai(status = 400)]
    InvalidName(PlainText<String>),

    #[oai(status = 403)]
    InternalAlias(PlainText<String>),
}

pub async fn delete(pool: &PgPool, name: &str) -> poem::Result<AliasDeleteResponse> {
    let normalized_alias = utils::normalize_name(name);

    if normalized_alias.len() == 0 {
        return Ok(AliasDeleteResponse::InvalidName(PlainText(
            "Invalid alias".to_string(),
        )));
    }

    let current_alias = sqlx::query_as::<_, Alias>(
        r#"
            SELECT *
            FROM alias
            WHERE alias = $1
            LIMIT 1
        "#,
    )
    .bind(&normalized_alias)
    .fetch_optional(pool)
    .await
    .map_err(InternalServerError)?;

    match current_alias {
        Some(current) => {
            if current.internal == true {
                return Ok(AliasDeleteResponse::InternalAlias(PlainText(format!(
                    "{} is an internal alias and can't be deleted.",
                    current.alias
                ))));
            }
        }
        None => {
            return Ok(AliasDeleteResponse::SuccessfullyDeleted(Json(
                DeleteCount { count: 0 },
            )))
        }
    };

    let delete_count = sqlx::query(
        r#"
        DELETE FROM alias
        WHERE alias = $1 AND internal = FALSE
        "#,
    )
    .bind(name)
    .execute(pool)
    .await
    .map_err(InternalServerError)?
    .rows_affected();

    let response = AliasDeleteResponse::SuccessfullyDeleted(Json(DeleteCount {
        count: delete_count,
    }));
    Ok(response)
}
