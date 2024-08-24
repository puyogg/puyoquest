use poem::{error::InternalServerError, web::Data, Result};
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    ApiResponse,
};
use sqlx::PgPool;

use super::Card;

#[derive(ApiResponse)]
pub enum GetByIdResponse {
    #[oai(status = 200)]
    Card(Json<Card>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

pub async fn get_by_id(pool: Data<&PgPool>, id: Path<String>) -> Result<GetByIdResponse> {
    let card: Option<Card> = sqlx::query_as(
        r#"
            SELECT *
            FROM card
            WHERE card_id = $1
            LIMIT 1
        "#,
    )
    .bind(&id.0)
    .fetch_optional(pool.0)
    .await
    .map_err(InternalServerError)?;

    match card {
        Some(c) => Ok(GetByIdResponse::Card(Json(c))),
        None => Ok(GetByIdResponse::NotFound(PlainText(format!(
            "Character with id {} not found",
            &id.0,
        )))),
    }
}
