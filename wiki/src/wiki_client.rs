mod fetch_raw_template;
pub use fetch_raw_template::*;

mod fetch_template;
pub use fetch_template::*;

pub struct WikiClient {
    client: reqwest::Client,
    api_url: String,
    base_url: String,
}

impl WikiClient {
    pub fn new(client: reqwest::Client, api_url: String, base_url: String) -> WikiClient {
        WikiClient {
            client,
            api_url,
            base_url,
        }
    }
}
