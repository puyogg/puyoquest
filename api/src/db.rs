pub async fn create_pool() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    sqlx::PgPool::connect("postgres://postgres:password@localhost:35432/ppq_api_db").await
}

pub struct PoolOpts<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub host: &'a str,
    pub port: &'a str,
    pub db: &'a str,
}

pub async fn create_pool_from_opts<'a>(opts: PoolOpts<'a>) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error>  {
    let PoolOpts { username, password, host, port, db } = opts;
    let connection_string = format!("postgres://{username}:{password}@{host}:{port}/{db}");

    sqlx::PgPool::connect(&connection_string).await
}
