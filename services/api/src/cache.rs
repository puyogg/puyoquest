pub async fn create_redis_connection(host: &str, port: &str) -> redis::aio::MultiplexedConnection {
    let redis_conn_url = format!("redis://:@{host}:{port}");

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_multiplexed_async_connection()
        .await
        .expect("failed to connect to redis")
}
