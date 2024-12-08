use api::db::create_pool;
use api::env_config::DeploymentEnvironment;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_pool(&DeploymentEnvironment::Local, &None).await?;
    let _ = sqlx::query("CREATE DATABASE admin_db")
        .execute(&pool)
        .await?;

    Ok(())
}
