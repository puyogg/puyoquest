use poem::{middleware::Cors, EndpointExt, IntoEndpoint, Route};
use poem_openapi::OpenApiService;

mod healthcheck;

pub fn init_api() -> impl IntoEndpoint {
    let api = OpenApiService::new(healthcheck::Healthcheck, "PPQ API", "0.1.0").server("http://localhost:3000");
    let ui = api.swagger_ui();
    let spec = api.spec();

    Route::new()
        .nest("/", api)
        .nest("/docs", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
}
