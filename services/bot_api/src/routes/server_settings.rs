use poem::{error::InternalServerError, web::Data};
use poem_openapi::{
    Object, OpenApi,
    param::Path,
    payload::{Json, PlainText},
};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Object, FromRow, Serialize)]
pub struct ServerSettings {
    pub server_id: String,
}

pub struct ServerSettingsRouter;
#[OpenApi(prefix_path = "/server-settings")]
impl ServerSettingsRouter {
    #[oai(path = "/:server_id", method = "get")]
    async fn fetch_server_settings(
        &self,
        pool: Data<&PgPool>,
        server_id: Path<String>,
    ) -> poem::Result<Json<Option<ServerSettings>>> {
        let pool = pool.0;

        let settings: Option<ServerSettings> = sqlx::query_as(
            r#"
                SELECT *
                FROM server_settings
                WHERE server_id = $1
            "#,
        )
        .bind(&server_id.0)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;

        Ok(Json(settings))
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
                INSERT INTO server_settings (server_id)
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

        let _ = sqlx::query(
            r#"
                DELETE FROM server_settings
                WHERE server_id = $1
            "#,
        )
        .bind(&server_id.0)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;

        Ok(PlainText(String::from("OK!!")))
    }
}
