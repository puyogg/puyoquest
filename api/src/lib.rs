use poem::{
    middleware::{Cors, CorsEndpoint},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use sqlx::{Pool, Postgres};

pub mod api_tag;
pub mod cards;
pub mod characters;
pub mod db;
pub mod healthcheck;

pub fn init_api(
    pool: Pool<Postgres>,
    wiki_api_url: String,
    wiki_base_url: String,
) -> poem::middleware::AddDataEndpoint<
    poem::middleware::AddDataEndpoint<CorsEndpoint<Route>, Pool<Postgres>>,
    wiki::wiki_client::WikiClient,
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

    let wiki_client =
        wiki::wiki_client::WikiClient::new(reqwest::Client::new(), wiki_api_url, wiki_base_url);

    Route::new()
        .nest("/", api)
        .nest("/docs", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
        .data(pool)
        .data(wiki_client)
}
