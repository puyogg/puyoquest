use poem::error::InternalServerError;
use poem_openapi::{payload::Json, ApiResponse};
use sqlx::PgPool;
use super::{Character, CharacterCreate};

#[derive(ApiResponse)]
pub enum UpsertResponse {
    #[oai(status = 200)]
    Ok(Json<Character>, #[oai(header = "Location")] String),
}

pub async fn upsert(pool: &PgPool, id: &str, character: &CharacterCreate) -> poem::Result<UpsertResponse> {
    let character: Result<Character, sqlx::Error> =
        sqlx::query_as(r#"
            INSERT INTO character (char_id, name, jp_name, link_name, main_color, side_color, type1, type2, voice_trans, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (char_id)
            DO UPDATE SET
                char_id = EXCLUDED.char_id,
                name = EXCLUDED.name,
                jp_name = EXCLUDED.jp_name,
                link_name = EXCLUDED.link_name,
                main_color = EXCLUDED.main_color,
                side_color = EXCLUDED.side_color,
                type1 = EXCLUDED.type1,
                type2 = EXCLUDED.type2,
                voice_trans = EXCLUDED.voice_trans,
                updated_at = EXCLUDED.updated_at
            RETURNING *
        "#)
        .bind(id)
        .bind(&character.name)
        .bind(&character.jp_name)
        .bind(&character.link_name)
        .bind(&character.main_color)
        .bind(&character.side_color)
        .bind(&character.type1)
        .bind(&character.type2)
        .bind(&character.voice_trans)
        .bind(&character.updated_at)
        .fetch_one(pool)
        .await;

    match character {
        Ok(c) => {
            let char_id = String::from(&c.char_id);
            // println!("upserted: {:?}", &c.to_json());
            Ok(UpsertResponse::Ok(Json(c), format!("/characters/{}", char_id)))
        },
        Err(e) => match e {
            sqlx::Error::Database(db_error) => {
                println!("{}", &db_error);
                Err(InternalServerError(db_error))
            },
            _ => Err(InternalServerError(e)),
        }
    }
}
