use poem_openapi::{ApiResponse, OpenApi, payload::PlainText};

#[derive(ApiResponse)]
enum HealthcheckResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>),
}

pub struct Healthcheck;
#[OpenApi(prefix_path = "/healthcheck")]
impl Healthcheck {
    #[oai(path = "/", method = "get")]
    async fn healthcheck(&self) -> HealthcheckResponse {
        HealthcheckResponse::Ok(PlainText(String::from("OK!!")))
    }
}
