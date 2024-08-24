use poem::{error::InternalServerError, Result};
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse,
};
use sqlx::PgPool;

use super::Alias;

#[derive(ApiResponse)]
pub enum ListByCharIdResponse {
    #[oai(status = 200)]
    Aliases(Json<Vec<Alias>>),

    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

pub async fn list_by_char_id(pool: &PgPool, char_id: &Option<String>) -> Result<ListByCharIdResponse> {
    let char_id: &str = match char_id {
        Some(c) => c,
        None => {
            return Ok(ListByCharIdResponse::BadRequest(PlainText("Missing char_id".to_string())));
        }
    };
    
    let aliases: Vec<Alias> = sqlx::query_as(r#"
        SELECT *
        FROM alias
        WHERE char_id = $1
    "#)
        .bind(char_id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;

    Ok(ListByCharIdResponse::Aliases(Json(aliases)))
}
