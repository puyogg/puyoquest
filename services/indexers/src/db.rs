use anyhow::anyhow;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    FromRow, PgPool,
};
use std::str::FromStr;

use crate::env::DeploymentEnvironment;

pub async fn create_pool(
    environment: &DeploymentEnvironment,
    db_connection_string: &str,
) -> Result<sqlx::Pool<sqlx::Postgres>, anyhow::Error> {
    match environment {
        DeploymentEnvironment::Local => {
            let p = sqlx::PgPool::connect(db_connection_string).await?;
            Ok(p)
        }
        DeploymentEnvironment::Production => {
            let options =
                PgConnectOptions::from_str(db_connection_string)?.ssl_mode(PgSslMode::Require);

            let pool = sqlx::PgPool::connect_with(options).await?;

            Ok(pool)
        }
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CronLastUpdated {
    pub task: String,
    pub updated_at: DateTime<Utc>,
}

pub async fn init_db(pool: &PgPool, default_days_ago: i64) -> Result<(), anyhow::Error> {
    let _ = sqlx::raw_sql("CREATE SCHEMA IF NOT EXISTS ppq_api_indexers")
        .execute(pool)
        .await?;
    let _ = sqlx::raw_sql(
        r#"CREATE TABLE IF NOT EXISTS ppq_api_indexers.cron_last_updated (
    task TEXT PRIMARY KEY,
    updated_at TIMESTAMPTZ NOT NULL
)"#,
    )
    .execute(pool)
    .await?;

    let last_run: Option<CronLastUpdated> = sqlx::query_as(
        r#"
        SELECT *
        FROM ppq_api_indexers.cron_last_updated
        WHERE ppq_api_indexers.cron_last_updated.task = 'recent_changes'
        LIMIT 1
    "#,
    )
    .fetch_optional(pool)
    .await?;

    if let None = last_run {
        let seven_days_ago = Utc::now() - Duration::days(default_days_ago);

        let inserted_last_run: CronLastUpdated = sqlx::query_as(
            r#"
            INSERT INTO ppq_api_indexers.cron_last_updated (
                task,
                updated_at
            )
            VALUES ($1, $2)
            ON CONFLICT (task)
            DO UPDATE SET
                task = EXCLUDED.task,
                updated_at = EXCLUDED.updated_at
            RETURNING *
        "#,
        )
        .bind("recent_changes")
        .bind(&seven_days_ago)
        .fetch_one(pool)
        .await?;

        println!(
            "Initializing last run for recent_changes: {}",
            seven_days_ago.to_rfc3339()
        );
        println!("{:?}", inserted_last_run);
    }

    Ok(())
}

pub async fn get_last_run(pool: &PgPool) -> Result<DateTime<Utc>, anyhow::Error> {
    let last_run: Option<CronLastUpdated> = sqlx::query_as(
        r#"
        SELECT *
        FROM ppq_api_indexers.cron_last_updated
        WHERE ppq_api_indexers.cron_last_updated.task = 'recent_changes'
        LIMIT 1
    "#,
    )
    .fetch_optional(pool)
    .await?;

    let last_run = last_run.ok_or(anyhow!("Missing last run!"))?;

    Ok(last_run.updated_at)
}

pub async fn update_last_run(pool: &PgPool, last_run: &DateTime<Utc>) -> Result<CronLastUpdated, anyhow::Error> {
    let inserted_last_run: CronLastUpdated = sqlx::query_as(
        r#"
        INSERT INTO ppq_api_indexers.cron_last_updated (
            task,
            updated_at
        )
        VALUES ($1, $2)
        ON CONFLICT (task)
        DO UPDATE SET
            task = EXCLUDED.task,
            updated_at = EXCLUDED.updated_at
        RETURNING *
    "#,
    )
    .bind("recent_changes")
    .bind(last_run)
    .fetch_one(pool)
    .await?;

    Ok(inserted_last_run)
}
