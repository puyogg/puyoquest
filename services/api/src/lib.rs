use std::sync::Arc;

use cache::RedisClient;
use poem::{
    middleware::{Cors, CorsEndpoint},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use sqlx::{Pool, Postgres};
use wiki::wiki_client::WikiClient;

pub mod aliases;
pub mod api_tag;
pub mod cache;
pub mod cards;
pub mod characters;
pub mod db;
pub mod healthcheck;
pub mod util;

pub type Api = poem::middleware::AddDataEndpoint<
    poem::middleware::AddDataEndpoint<
        poem::middleware::AddDataEndpoint<CorsEndpoint<Route>, Pool<Postgres>>,
        WikiClient,
    >,
    Arc<RedisClient>,
>;

pub fn init_api(
    pool: Pool<Postgres>,
    wiki_client: WikiClient,
    redis_client: Arc<RedisClient>,
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
        .data(pool)
        .data(wiki_client)
        .data(redis_client)
}
