use poem::{error::InternalServerError, web::Data};
use poem_openapi::{
    Object, OpenApi,
    param::{Path, Query},
    payload::{Json, PlainText},
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Object, FromRow, Serialize, Deserialize)]
pub struct UserRanking {
    pub user_id: String,
    pub server_id: String,
    pub game_type: String,
    pub correct: i32,
    pub ranking: i64,
}

pub struct Leaderboard;
#[OpenApi(prefix_path = "/leaderboards")]
impl Leaderboard {
    /// Fetch the top 10 players for a game type on a server
    #[oai(path = "/:server_id/:game_type/top", method = "get")]
    async fn server_game_top_10(
        &self,
        pool: Data<&PgPool>,
        server_id: Path<String>,
        game_type: Path<String>,
    ) -> poem::Result<Json<Vec<UserRanking>>> {
        let pool = pool.0;
        let rankings: Vec<UserRanking> = sqlx::query_as(
            r#"
                SELECT *, row_number() OVER (ORDER BY correct DESC, updated_at ASC) AS ranking
                FROM bot.leaderboard
                WHERE
                    server_id = $1
                    AND game_type = $2
                LIMIT 10
            "#,
        )
        .bind(&server_id.0)
        .bind(&game_type.0)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;

        Ok(Json(rankings))
    }

    /// Fetch a user's ranking and the 9 players surrounding them.
    /// This tries to show 10 users unless the leaderboard+game currently has less
    /// than 10 players.
    #[oai(path = "/:server_id/:game_type/window", method = "get")]
    async fn server_game_user_window(
        &self,
        pool: Data<&PgPool>,
        server_id: Path<String>,
        game_type: Path<String>,
        user_id: Query<String>,
    ) -> poem::Result<Json<Vec<UserRanking>>> {
        let pool = pool.0;
        let rankings: Vec<UserRanking> = sqlx::query_as(
            r#"
                WITH full_leaderboard AS (
                    SELECT *, row_number() OVER (ORDER BY correct DESC, updated_at ASC) AS ranking
                    FROM bot.leaderboard
                    WHERE
                        game_type = $2
                        AND server_id = $1
                ), user_ranking AS (
                    SELECT full_leaderboard.ranking as ranking
                    FROM full_leaderboard
                    WHERE
                        user_id = $3
                    LIMIT 1
                ), player_count AS (
                    SELECT COUNT(*) as count
                    FROM full_leaderboard
                ), ranking_range AS (
                    SELECT
                        GREATEST(((user_ranking.ranking - 5) - (GREATEST(4 - (player_count.count - user_ranking.ranking), 0))), 1) as start,
                        LEAST(((user_ranking.ranking + 4) + (GREATEST(6 - user_ranking.ranking, 0))), player_count.count) as end
                    FROM user_ranking
                    CROSS JOIN player_count
                )
                SELECT full_leaderboard.*
                FROM full_leaderboard
                WHERE
                    full_leaderboard.ranking >= (SELECT ranking_range.start FROM ranking_range)
                    AND full_leaderboard.ranking <= (SELECT ranking_range.end FROM ranking_range);
            "#
        )
        .bind(&server_id.0)
        .bind(&game_type.0)
        .bind(&user_id.0)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;

        Ok(Json(rankings))
    }

    /// Get the player count for a game and server
    #[oai(path = "/:server_id/:game_type/player_count", method = "get")]
    async fn server_game_player_count(
        &self,
        pool: Data<&PgPool>,
        server_id: Path<String>,
        game_type: Path<String>,
    ) -> poem::Result<Json<i64>> {
        let pool = pool.0;

        let count: i64 = sqlx::query_scalar(
            r#"
                SELECT COUNT(*)
                FROM bot.leaderboard
                WHERE
                    server_id = $1
                    AND game_type = $2
            "#,
        )
        .bind(&server_id.0)
        .bind(&game_type.0)
        .fetch_one(pool)
        .await
        .map_err(InternalServerError)?;

        Ok(Json(count))
    }

    /// Upsert a user ranking and increment their score
    #[oai(path = "/:server_id/:game_type/:user_id/increment", method = "post")]
    async fn increment_score(
        &self,
        pool: Data<&PgPool>,
        server_id: Path<String>,
        game_type: Path<String>,
        user_id: Path<String>,
    ) -> poem::Result<Json<UserRanking>> {
        let pool = pool.0;

        let _ = sqlx::query(
            r#"
                INSERT INTO bot.leaderboard (game_type, user_id, server_id, correct)
                VALUES ($2, $3, $1, 1)
                ON CONFLICT (user_id, server_id, game_type)
                DO UPDATE
                    SET correct = bot.leaderboard.correct + 1
                    WHERE bot.leaderboard.user_id = EXCLUDED.user_id
                      AND bot.leaderboard.server_id = EXCLUDED.server_id
                      AND bot.leaderboard.game_type = EXCLUDED.game_type
            "#,
        )
        .bind(&server_id.0)
        .bind(&game_type.0)
        .bind(&user_id.0)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;

        let new_ranking: UserRanking = sqlx::query_as(
            r#"
                WITH ranked_leaderboard AS (
                    SELECT *, row_number() OVER (ORDER BY correct DESC) AS ranking
                    FROM bot.leaderboard
                    WHERE
                        game_type = $2
                        AND server_id = $1
                )
                SELECT *
                FROM ranked_leaderboard
                WHERE
                    server_id = $1
                    AND game_type = $2
                    AND user_id = $3
            "#,
        )
        .bind(&server_id.0)
        .bind(&game_type.0)
        .bind(&user_id.0)
        .fetch_one(pool)
        .await
        .map_err(InternalServerError)?;

        Ok(Json(new_ranking))
    }

    /// Delete all the results for a game_type across all servers.
    /// I don't want to hold Discord user data for very long.
    #[oai(path = "/:game_type", method = "delete")]
    async fn reset_everyones_games(
        &self,
        pool: Data<&PgPool>,
        game_type: Path<String>,
    ) -> poem::Result<PlainText<String>> {
        let pool = pool.0;

        let _ = sqlx::query(
            r#"
                DELETE FROM bot.leaderboard
                WHERE game_type = $1
            "#,
        )
        .bind(&game_type.0)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;

        Ok(PlainText(String::from("OK!")))
    }
}
