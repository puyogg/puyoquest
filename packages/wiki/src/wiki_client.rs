mod fetch_raw_template;
pub use fetch_raw_template::*;

mod fetch_template;
pub use fetch_template::*;

mod card_categories;
pub use card_categories::*;

mod page_image_filenames;
pub use page_image_filenames::*;

mod image_url;
pub use image_url::*;

mod recent_char_changes;
pub use recent_char_changes::*;

#[derive(Clone)]
pub struct WikiClient {
    client: reqwest::Client,
    api_url: String,
    base_url: String,
}

impl WikiClient {
    pub fn new(api_url: impl Into<String>, base_url: impl Into<String>) -> WikiClient {
        let client = reqwest::Client::new();
        let api_url = api_url.into();
        let base_url = base_url.into();

        WikiClient {
            client,
            api_url,
            base_url,
        }
    }
}
