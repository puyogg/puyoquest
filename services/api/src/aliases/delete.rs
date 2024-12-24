use poem::error::InternalServerError;
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse, Object,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
}

pub async fn delete(pool: &PgPool, name: &str) -> poem::Result<AliasDeleteResponse> {
    let normalized_alias = utils::normalize_name(name);

    if normalized_alias.len() == 0 {
        return Ok(AliasDeleteResponse::InvalidName(PlainText(
            "Invalid alias".to_string(),
        )));
    }

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
