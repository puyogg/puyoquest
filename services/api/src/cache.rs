#[derive(Debug)]
pub struct RedisClient {
    pub conn: redis::aio::MultiplexedConnection,
    prefix: String,
}

impl RedisClient {
    pub fn prefixed(&self, key: &str) -> String {
        format!("{}{key}", &self.prefix)
    }
}

pub const DEFAULT_REDIS_KEY_PREFIX: &'static str = "";

pub async fn create_redis_connection(host: &str, port: &str, prefix: String) -> RedisClient {
    let redis_conn_url = format!("redis://:@{host}:{port}");

    let conn = redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_multiplexed_async_connection()
        .await
        .expect("failed to connect to redis");

    RedisClient { conn, prefix }
}
