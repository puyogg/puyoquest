use poem_openapi::{payload::PlainText, ApiResponse, OpenApi, Tags};

#[derive(ApiResponse)]
enum HealthcheckResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>),
}

#[derive(Tags)]
enum ApiTag {
    Healthcheck,
}

pub struct Healthcheck;
#[OpenApi(prefix_path = "/healthcheck", tag = "ApiTag::Healthcheck" )]
impl Healthcheck {
    #[oai(path = "/", method = "get")]
    async fn healthcheck(&self) -> HealthcheckResponse {
        HealthcheckResponse::Ok(PlainText(String::from("OK!")))
    }
}
