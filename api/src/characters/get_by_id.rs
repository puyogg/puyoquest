use poem::{web::Data, Result, error::InternalServerError};
use poem_openapi::{param::Path, payload::{Json, PlainText}, ApiResponse};
use sqlx::PgPool;

use super::Character;

#[derive(ApiResponse)]
pub enum GetByIdResponse {
    #[oai(status = 200)]
    Character(Json<Character>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

pub async fn get_by_id(pool: Data<&PgPool>, id: Path<String>) -> Result<GetByIdResponse> {
    let character: Option<Character> =
        sqlx::query_as("
            SELECT *
            FROM character
            WHERE char_id = $1
        ")
        .bind(&id.0)
        .fetch_optional(pool.0)
        .await
        .map_err(InternalServerError)?;

    match character {
        Some(c) => {
            // println!("Fetched timestamp {:?}", c.updated_at);
            // let json = Json(&c).to_json_string();
            // println!("{json}");
            Ok(GetByIdResponse::Character(Json(c)))
        },
        None => Ok(GetByIdResponse::NotFound(PlainText(format!(
            "Character with id {} not found",
            &id.0,
        ))))
    }
}
