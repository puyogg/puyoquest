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

mod resolve_wiki_text;
pub use resolve_wiki_text::*;

mod fetch_character_series;
pub use fetch_character_series::*;

mod character_card_ids;
pub use character_card_ids::*;

mod all_ppq_categories;
pub use all_ppq_categories::*;

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
