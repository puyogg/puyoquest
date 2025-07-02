use poem::{error::InternalServerError, web::Data};
use poem_openapi::{
    Object, OpenApi,
    param::Path,
    payload::{Json, PlainText},
};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Object, FromRow, Serialize)]
pub struct LeaderboardChannel {
    pub server_id: String,
    pub game_type: String,
    pub channel_id: String,
}

pub struct LeaderboardChannelRouter;
#[OpenApi(prefix_path = "/leaderboard-channel")]
impl LeaderboardChannelRouter {
    #[oai(path = "/:server_id/:game_type", method = "get")]
    async fn get_leaderboard_channel(
        &self,
        pool: Data<&PgPool>,
        server_id: Path<String>,
        game_type: Path<String>,
    ) -> poem::Result<Json<Option<LeaderboardChannel>>> {
        let pool = pool.0;

        let leaderboard_channel: Option<LeaderboardChannel> = sqlx::query_as(
            r#"
                SELECT *
                FROM bot.leaderboard_channel
                WHERE server_id = $1 AND game_type = $2
            "#,
        )
        .bind(&server_id.0)
        .bind(&game_type.0)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;

        Ok(Json(leaderboard_channel))
    }

    #[oai(path = "/:server_id/:game_type", method = "post")]
    async fn set_leaderboard_channel(
        &self,
        pool: Data<&PgPool>,
        server_id: Path<String>,
        game_type: Path<String>,
    ) -> poem::Result<Json<LeaderboardChannel>> {
        let pool = pool.0;

        let leaderboard_channel: LeaderboardChannel = sqlx::query_as(
            r#"
                INSERT INTO bot.leaderboard_channel (server_id, game_type, channel_id)
                VALUES ($1, $2, $3)
                ON CONFLICT (server_id, game_type)
                DO UPDATE
                    SET channel_id = $3
                    WHERE bot.leaderboard_channel.server_id = EXCLUDED.server_id
                      AND bot.leaderboard_channel.game_type = EXCLUDED.game_type
                RETURNING *
            "#,
        )
        .bind(&server_id.0)
        .bind(&game_type.0)
        .fetch_one(pool)
        .await
        .map_err(InternalServerError)?;

        Ok(Json(leaderboard_channel))
    }

    #[oai(path = "/:server_id/:game_type", method = "delete")]
    async fn delete_leaderboard_channel(
        &self,
        pool: Data<&PgPool>,
        server_id: Path<String>,
        game_type: Path<String>,
    ) -> poem::Result<PlainText<String>> {
        let pool = pool.0;

        let _ = sqlx::query(
            r#"
                DELETE FROM bot.leaderboard_channel
                WHERE server_id = $1 AND game_type = $2
            "#,
        )
        .bind(&server_id.0)
        .bind(&game_type.0)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;

        Ok(PlainText(String::from("OK!!")))
    }
}
