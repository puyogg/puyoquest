use poem::error::InternalServerError;
use poem_openapi::{payload::PlainText, ApiResponse};
use sqlx::PgPool;

#[derive(ApiResponse)]
pub enum AliasDeleteResponse {
    #[oai(status = 200)]
    SuccessfullyDeleted(PlainText<String>),

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
        WHERE alias = $1
        "#,
    )
    .bind(name)
    .execute(pool)
    .await
    .map_err(InternalServerError)?
    .rows_affected();

    let delete_count = delete_count.to_string();

    let response = AliasDeleteResponse::SuccessfullyDeleted(PlainText(delete_count));
    Ok(response)
}
