use poem::{
    middleware::{Cors, CorsEndpoint},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use sqlx::{Pool, Postgres};
use wiki::wiki_client::WikiClient;

pub mod api_tag;
pub mod cards;
pub mod characters;
pub mod db;
pub mod healthcheck;
pub mod aliases;

pub fn init_api(
    pool: Pool<Postgres>,
    wiki_client: WikiClient,
) -> poem::middleware::AddDataEndpoint<
    poem::middleware::AddDataEndpoint<CorsEndpoint<Route>, Pool<Postgres>>,
    WikiClient,
> {
    let api = OpenApiService::new(
        (
            healthcheck::Healthcheck,
            characters::CharactersRoute,
            cards::CardsRouter,
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
}
