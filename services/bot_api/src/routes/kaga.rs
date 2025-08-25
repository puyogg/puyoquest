use poem::{error::InternalServerError, web::Data};
use poem_openapi::{
    ApiResponse, Object, OpenApi,
    param::Query,
    payload::{Json, PlainText},
};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Object, FromRow, Serialize)]
pub struct KagaData {
    pub kaga_id: String,
    pub url: String,
}

#[derive(ApiResponse)]
enum KagaResponse {
    #[oai(status = 200)]
    Kaga(Json<KagaData>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

pub struct Kaga;
#[OpenApi(prefix_path = "/kaga")]
impl Kaga {
    /// Get a kaga image url
    #[oai(path = "/", method = "get")]
    async fn get_by_id(
        &self,
        pool: Data<&PgPool>,
        id: Query<String>,
    ) -> poem::Result<KagaResponse> {
        let id = id.0;

        let kaga_data: Option<KagaData> = sqlx::query_as(
            r#"
                SELECT *
                FROM bot.kaga
                WHERE kaga_id = $1
            "#,
        )
        .bind(&id)
        .fetch_optional(pool.0)
        .await
        .map_err(InternalServerError)?;

        match kaga_data {
            Some(k) => return Ok(KagaResponse::Kaga(Json(k))),
            None => {
                return Ok(KagaResponse::NotFound(PlainText(format!(
                    "kaga_id {id} does not exist"
                ))));
            }
        }
    }

    /// Get a random kaga image
    #[oai(path = "/random", method = "get")]
    async fn get_random(&self, pool: Data<&PgPool>) -> poem::Result<KagaResponse> {
        let kaga_data: Option<KagaData> = sqlx::query_as(
            r#"
                SELECT *
                FROM bot.kaga
                ORDER BY random()
                LIMIT 1
            "#,
        )
        .fetch_optional(pool.0)
        .await
        .map_err(InternalServerError)?;

        match kaga_data {
            Some(k) => return Ok(KagaResponse::Kaga(Json(k))),
            None => {
                return Ok(KagaResponse::NotFound(PlainText(format!(
                    "Failed to find a kaga image"
                ))));
            }
        }
    }

    /// Set a kaga image url
    #[oai(path = "/", method = "put")]
    async fn set_kaga_image(
        &self,
        pool: Data<&PgPool>,
        kaga: Json<KagaData>,
    ) -> poem::Result<Json<KagaData>> {
        let k: KagaData = sqlx::query_as(
            r#"
                INSERT INTO bot.kaga (kaga_id, url)
                VALUES ($1, $2)
                ON CONFLICT (kaga_id)
                DO UPDATE SET
                    kaga_id = EXCLUDED.kaga_id,
                    url = EXCLUDED.url
                RETURNING *
            "#,
        )
        .bind(&kaga.0.kaga_id)
        .bind(&kaga.0.url)
        .fetch_one(pool.0)
        .await
        .map_err(InternalServerError)?;

        Ok(Json(k))
    }

    /// Delete a kaga image url
    #[oai(path = "/", method = "delete")]
    async fn delete_kaga_image(&self, pool: Data<&PgPool>, id: Query<String>) -> poem::Result<()> {
        let _ = sqlx::query(
            r#"
                DELETE FROM bot.kaga
                WHERE kaga_id = $1
            "#,
        )
        .bind(&id.0)
        .execute(pool.0)
        .await
        .map_err(InternalServerError)?;

        Ok(())
    }
}
