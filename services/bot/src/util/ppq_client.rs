use reqwest::Client;
use crate::env_config::ENV;
use std::sync::LazyLock;

pub static PPQ_API_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    reqwest::Client::new()
});

pub static PPQ_API_HOST: LazyLock<String> = LazyLock::new(|| String::from(&*ENV.ppq_api_host));
