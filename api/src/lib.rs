use poem::{middleware::{Cors, CorsEndpoint}, EndpointExt, Route};
use poem_openapi::OpenApiService;
use sqlx::{Pool, Postgres};

pub mod characters;
pub mod healthcheck;
pub mod cards;
pub mod api_tag;
pub mod db;

pub fn init_api(pool: Pool<Postgres>) -> poem::middleware::AddDataEndpoint<CorsEndpoint<Route>, Pool<Postgres>> {
    let api = OpenApiService::new(
        (healthcheck::Healthcheck, characters::CharactersRoute, cards::CardsRouter),
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
}
