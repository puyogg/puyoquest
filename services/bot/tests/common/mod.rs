use sdk::apis::configuration::Configuration as PpqApiConfiguration;

pub fn get_ppq_api_config() -> PpqApiConfiguration {
    PpqApiConfiguration {
        base_path: "http://localhost:3000".to_string(),
        user_agent: None,
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: None,
    }
}
