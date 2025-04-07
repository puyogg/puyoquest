use poem::{error::InternalServerError, web::Data};
use poem_openapi::{ApiResponse, Object, OpenApi, param::Query, payload::PlainText};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Object, FromRow, Serialize)]
pub struct KagaData {
    kaga_id: String,
    url: String,
}

#[derive(ApiResponse)]
enum GetByIdResponse {
    #[oai(status = 200)]
    Url(PlainText<String>),

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
    ) -> poem::Result<GetByIdResponse> {
        let id = id.0;

        let kaga_data: Option<KagaData> = sqlx::query_as(
            r#"
                SELECT *
                FROM kaga
                WHERE kaga_id = $1
            "#,
        )
        .bind(&id)
        .fetch_optional(pool.0)
        .await
        .map_err(InternalServerError)?;

        match kaga_data {
            Some(k) => return Ok(GetByIdResponse::Url(PlainText(k.url))),
            None => {
                return Ok(GetByIdResponse::NotFound(PlainText(format!(
                    "kaga_id {id} does not exist"
                ))));
            }
        }
    }

    /// Set a kaga image url
    #[oai(path = "/", method = "put")]
    async fn set_kaga_image(
        &self,
        pool: Data<&PgPool>,
        id: Query<String>,
        url: Query<String>,
    ) -> poem::Result<()> {
        let _: KagaData = sqlx::query_as(
            r#"
                INSERT INTO kaga (kaga_id, url)
                VALUES ($1, $2)
                ON CONFLICT (kaga_id)
                DO UPDATE SET
                    kaga_id = EXCLUDED.kaga_id,
                    url = EXCLUDED.url
                RETURNING *
            "#,
        )
        .bind(&id.0)
        .bind(&url.0)
        .fetch_one(pool.0)
        .await
        .map_err(InternalServerError)?;

        Ok(())
    }

    /// Delete a kaga image url
    #[oai(path = "/", method = "delete")]
    async fn delete_kaga_image(&self, pool: Data<&PgPool>, id: Query<String>) -> poem::Result<()> {
        let _ = sqlx::query(
            r#"
                DELETE FROM kaga
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
