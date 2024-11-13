use poem::error::InternalServerError;

#[derive(Debug)]
struct ParameterStoreFetchError {}
impl std::error::Error for ParameterStoreFetchError {}
impl std::fmt::Display for ParameterStoreFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to fetch value from parameter store.")
    }
}

pub async fn fetch_parameter(ssm_client: &aws_sdk_ssm::Client, name: &str) -> Result<String, poem::Error> {
    ssm_client
        .get_parameter()
        .name(name)
        .send()
        .await
        .map_err(InternalServerError)
        .map(|p| p.parameter)?
        .and_then(|p| p.value)
        .ok_or(ParameterStoreFetchError {})
        .map_err(InternalServerError)
}
