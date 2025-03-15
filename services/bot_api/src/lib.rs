use std::sync::Arc;

use aws::AwsClient;
use poem::{
    EndpointExt, Route,
    middleware::{AddDataEndpoint, Cors, CorsEndpoint},
};
use poem_openapi::OpenApiService;
use sqlx::{Pool, Postgres};

pub mod aws;
pub mod db;
pub mod env;
mod routes;

pub type BotApi =
    AddDataEndpoint<AddDataEndpoint<CorsEndpoint<Route>, Arc<AwsClient>>, Pool<Postgres>>;

pub fn init_api(aws_client: Arc<AwsClient>, pool: Pool<Postgres>) -> BotApi {
    let api = OpenApiService::new(
        (
            routes::healthcheck::Healthcheck,
            routes::leaderboard::Leaderboard,
            routes::server_settings::ServerSettingsRouter,
            routes::leaderboard_channel::LeaderboardChannelRouter,
        ),
        "Yotarou API",
        "0.1.0",
    )
    .server("http://localhost:3001");
    let ui = api.swagger_ui();
    let spec = api.spec();

    Route::new()
        .nest("/", api)
        .nest("/docs", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
        .data(aws_client)
        .data(pool)
}
