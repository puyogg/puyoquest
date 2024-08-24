use crate::util::mediawiki_types::MediawikiQuery;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct ImageInfoItem {
    pub url: String,
}

#[derive(Deserialize)]
pub struct ImageUrlPage {
    pub pageid: i32,
    pub ns: i32,
    pub title: String,
    pub imagerepository: String,
    pub imageinfo: Vec<ImageInfoItem>,
}

type ImageUrlResponse = MediawikiQuery<Value, ImageUrlPage>;

#[async_trait]
pub trait ImageUrl {
    async fn image_url<T: AsRef<str> + Send>(&self, file_page: T) -> Result<Option<String>, reqwest::Error>;
}

#[async_trait]
impl ImageUrl for super::WikiClient {
    async fn image_url<T: AsRef<str> + Send>(&self, file_page: T) -> Result<Option<String>, reqwest::Error> {
        let response = self
            .client
            .get(&self.api_url)
            .query(&[
                ("action", "query"),
                ("format", "json"),
                ("formatversion", "2"),
                ("prop", "imageinfo"),
                ("iiprop", "url"),
                ("titles", file_page.as_ref()),
            ])
            .send()
            .await?
            .json::<ImageUrlResponse>()
            .await?;

        let first_page = response.query.pages.get(0);

        let url: Option<String> = first_page
            .and_then(|page| page.imageinfo.get(0))
            .and_then(|imageinfo| Some(imageinfo.url.clone()));

        Ok(url)
    }
}
