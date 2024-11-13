use std::sync::Arc;

use aws::s3::S3BackupClient;
use cache::RedisClient;
use config::ApiConfig;
use poem::{
    middleware::{Cors, CorsEndpoint},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use sqlx::{Pool, Postgres};
use wiki::wiki_client::WikiClient;

pub mod aliases;
pub mod api_tag;
pub mod aws;
pub mod cache;
pub mod cards;
pub mod characters;
pub mod config;
pub mod db;
pub mod healthcheck;
pub mod util;

pub type Api = poem::middleware::AddDataEndpoint<
    poem::middleware::AddDataEndpoint<
        poem::middleware::AddDataEndpoint<
            poem::middleware::AddDataEndpoint<
                poem::middleware::AddDataEndpoint<CorsEndpoint<Route>, Arc<ApiConfig>>,
                Pool<Postgres>,
            >,
            WikiClient,
        >,
        Arc<RedisClient>,
    >,
    Arc<S3BackupClient>,
>;

pub fn init_api(
    api_config: Arc<ApiConfig>,
    pool: Pool<Postgres>,
    wiki_client: WikiClient,
    redis_client: Arc<RedisClient>,
    s3_client: Arc<S3BackupClient>,
) -> Api {
    let api = OpenApiService::new(
        (
            healthcheck::Healthcheck,
            characters::CharactersRoute,
            cards::CardsRouter,
            aliases::AliasesRouter,
        ),
        "PPQ API",
        "0.1.0",
    )
    .server("http://localhost:3000");
    let ui = api.swagger_ui();
    let spec = api.spec();

    Route::new()
        .nest("/", api)
        .nest("/docs", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
        .data(api_config)
        .data(pool)
        .data(wiki_client)
        .data(redis_client)
        .data(s3_client)
}
