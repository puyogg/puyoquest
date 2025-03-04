use std::str::FromStr;

use sqlx::postgres::PgConnectOptions;

use crate::env::DeploymentEnvironment;

pub async fn create_pool(
    environment: &DeploymentEnvironment,
    db_connection_string: &str,
) -> Result<sqlx::Pool<sqlx::Postgres>, Box<dyn std::error::Error>> {
    let pool: sqlx::Pool<sqlx::Postgres> = match environment {
        DeploymentEnvironment::Local => sqlx::PgPool::connect(db_connection_string).await?,
        DeploymentEnvironment::Production => {
            let options = PgConnectOptions::from_str(db_connection_string)?
                .ssl_mode(sqlx::postgres::PgSslMode::Require);
            sqlx::PgPool::connect_with(options).await?
        }
    };

    Ok(pool)
}
