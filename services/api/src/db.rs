use std::str::FromStr;

use sqlx::postgres::PgConnectOptions;

use crate::aws::ssm;
use crate::env_config::DeploymentEnvironment;

pub async fn create_pool(
    environment: &DeploymentEnvironment,
    db_connection_string: &Option<String>,
) -> Result<sqlx::Pool<sqlx::Postgres>, Box<dyn std::error::Error>> {
    match environment {
        DeploymentEnvironment::Local => match db_connection_string {
            Some(s) => Ok(sqlx::PgPool::connect(s).await?),
            None => Ok(sqlx::PgPool::connect(
                "postgres://postgres:password@localhost:35432/ppq_api_db",
            )
            .await?),
        },
        DeploymentEnvironment::Production => {
            let connection_string = match db_connection_string {
                Some(s) => s.clone(),
                None => {
                    let sdk_config = aws_config::from_env().load().await;
                    let ssm_client = aws_sdk_ssm::Client::new(&sdk_config);

                    ssm::fetch_parameter(&ssm_client, "PPQ_DB_CONNECTION_STRING").await?
                }
            };

            let options = PgConnectOptions::from_str(&connection_string)?
                .ssl_mode(sqlx::postgres::PgSslMode::Require);
            let pool = sqlx::PgPool::connect_with(options).await?;

            Ok(pool)
        }
    }
}

pub struct PoolOpts<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub host: &'a str,
    pub port: &'a str,
    pub db: &'a str,
}

pub async fn create_pool_from_opts<'a>(
    opts: PoolOpts<'a>,
) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let PoolOpts {
        username,
        password,
        host,
        port,
        db,
    } = opts;
    let connection_string = format!("postgres://{username}:{password}@{host}:{port}/{db}");

    sqlx::PgPool::connect(&connection_string).await
}
