use poem::error::InternalServerError;
use poem_openapi::{payload::Json, ApiResponse};
use sqlx::PgPool;

use super::{Card, CardCreate};

#[derive(ApiResponse)]
pub enum UpsertResponse {
    #[oai(status = 200)]
    Ok(Json<Card>, #[oai(header = "Location")] String),
}

pub async fn upsert(pool: &PgPool, card: &CardCreate) -> poem::Result<UpsertResponse> {
    let inserted_card: Result<Card, sqlx::Error> = sqlx::query_as(
        r#"
            INSERT INTO card (
                card_id,
                char_id,
                rarity,
                rarity_modifier,
                name,
                name_normalized,
                jp_name,
                jp_name_normalized,
                link_name,
                link_name_normalized,
                card_type,
                main_color,
                side_color,
                wiki_template,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            ON CONFLICT (card_id)
            DO UPDATE SET
                card_id = EXCLUDED.card_id,
                char_id = EXCLUDED.char_id,
                rarity = EXCLUDED.rarity,
                rarity_modifier = EXCLUDED.rarity_modifier,
                name = EXCLUDED.name,
                name_normalized = EXCLUDED.name_normalized,
                jp_name = EXCLUDED.jp_name,
                jp_name_normalized = EXCLUDED.jp_name_normalized,
                link_name = EXCLUDED.link_name,
                link_name_normalized = EXCLUDED.link_name_normalized,
                card_type = EXCLUDED.card_type,
                main_color = EXCLUDED.main_color,
                side_color = EXCLUDED.side_color,
                wiki_template = EXCLUDED.wiki_template,
                updated_at = EXCLUDED.updated_at
            RETURNING *
        "#,
    )
    .bind(&card.card_id)
    .bind(&card.char_id)
    .bind(&card.rarity)
    .bind(&card.rarity_modifier)
    .bind(&card.name)
    .bind(&card.name_normalized)
    .bind(&card.jp_name)
    .bind(&card.jp_name_normalized)
    .bind(&card.link_name)
    .bind(&card.link_name_normalized)
    .bind(&card.card_type)
    .bind(&card.main_color)
    .bind(&card.side_color)
    .bind(&card.wiki_template)
    .bind(&card.updated_at)
    .fetch_one(pool)
    .await;

    match inserted_card {
        Ok(c) => {
            let card_id = String::from(&c.card_id);
            Ok(UpsertResponse::Ok(Json(c), format!("/cards/{card_id}")))
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
