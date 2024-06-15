use poem::{error::InternalServerError, web::Data, Result};
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    ApiResponse,
};
use sqlx::PgPool;
use wiki::wiki_client::{FetchTemplate, FetchTemplateError};

use super::{types::CharacterCreate, upsert, Character};

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
            // println!("Fetched timestamp {:?}", c.updated_at);
            // let json = Json(&c).to_json_string();
            // println!("{json}");
            Ok(GetByIdResponse::Character(Json(c)))
        }
        None => Ok(GetByIdResponse::NotFound(PlainText(format!(
            "Character with id {} not found",
            &id,
        )))),
    }
}
