use crate::api_tag::ApiTag;
use poem_openapi::{payload::PlainText, ApiResponse, OpenApi};

#[derive(ApiResponse)]
enum HealthcheckResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>),
}

pub struct Healthcheck;
#[OpenApi(prefix_path = "/healthcheck", tag = "ApiTag::Healthcheck")]
impl Healthcheck {
    #[oai(path = "/", method = "get")]
    async fn healthcheck(&self) -> HealthcheckResponse {
        HealthcheckResponse::Ok(PlainText(String::from("OK!")))
    }
}
