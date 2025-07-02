use poem::{error::InternalServerError, web::Data};
use poem_openapi::{
    ApiResponse, Object, OpenApi,
    param::Path,
    payload::{Json, PlainText},
};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Object, FromRow, Serialize)]
pub struct ServerSettings {
    pub server_id: String,
}

#[derive(ApiResponse)]
pub enum FetchSettingsByIdResponse {
    #[oai(status = 200)]
    Settings(Json<ServerSettings>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

pub struct ServerSettingsRouter;
#[OpenApi(prefix_path = "/server-settings")]
impl ServerSettingsRouter {
    #[oai(path = "/:server_id", method = "get")]
    async fn fetch_server_settings(
        &self,
        pool: Data<&PgPool>,
        server_id: Path<String>,
    ) -> poem::Result<FetchSettingsByIdResponse> {
        let pool = pool.0;

        let settings: Option<ServerSettings> = sqlx::query_as(
            r#"
                SELECT *
                FROM bot.server_settings
                WHERE server_id = $1
            "#,
        )
        .bind(&server_id.0)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;

        // Ok(Json(settings))
        match settings {
            Some(settings) => Ok(FetchSettingsByIdResponse::Settings(Json(settings))),
            None => Ok(FetchSettingsByIdResponse::NotFound(PlainText(format!(
                "Failed to find settings for server_id: {}",
                &server_id.0
            )))),
        }
    }

    #[oai(path = "/", method = "post")]
    async fn upsert_server_settings(
        &self,
        pool: Data<&PgPool>,
        settings: Json<ServerSettings>,
    ) -> poem::Result<Json<ServerSettings>> {
        let pool = pool.0;

        // TODO: update some actual settings
        let server_settings: ServerSettings = sqlx::query_as(
            r#"
                INSERT INTO bot.server_settings (server_id)
                VALUES ($1)
                ON CONFLICT (server_id) DO UPDATE
                    SET server_id = EXCLUDED.server_id
                RETURNING *
            "#,
        )
        .bind(&settings.0.server_id)
        .fetch_one(pool)
        .await
        .map_err(InternalServerError)?;

        Ok(Json(server_settings))
    }

    #[oai(path = "/:server_id", method = "delete")]
    async fn delete_server(
        &self,
        pool: Data<&PgPool>,
        server_id: Path<String>,
    ) -> poem::Result<PlainText<String>> {
        let pool = pool.0;

        let delete_count = sqlx::query(
            r#"
                DELETE FROM bot.server_settings
                WHERE server_id = $1
            "#,
        )
        .bind(&server_id.0)
        .execute(pool)
        .await
        .map_err(InternalServerError)?
        .rows_affected();
        println!("Rows affected: {delete_count}");

        Ok(PlainText(String::from("OK!!")))
    }
}
