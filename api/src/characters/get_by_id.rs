use chrono::Utc;
use poem::{error::InternalServerError, Result};
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse,
};
use sqlx::PgPool;

use super::Character;

#[derive(ApiResponse)]
pub enum GetByIdResponse {
    #[oai(status = 200)]
    Character(Json<Character>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

pub async fn get_by_id(pool: &PgPool, id: &str) -> Result<GetByIdResponse> {
    let character: Option<Character> = sqlx::query_as(
        "
            SELECT *
            FROM character
            WHERE char_id = $1
        ",
    )
    .bind(&id)
    .fetch_optional(pool)
    .await
    .map_err(InternalServerError)?;

    match character {
        Some(c) => {
            let now = Utc::now();
            let diff = now.signed_duration_since(c.updated_at);
            if diff.num_minutes() >= 10i64 {
                println!("Should update character!");
            }

            Ok(GetByIdResponse::Character(Json(c)))
        }
        None => Ok(GetByIdResponse::NotFound(PlainText(format!(
            "Character with id {} not found",
            &id,
        )))),
    }
}
