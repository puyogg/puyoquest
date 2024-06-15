use async_trait::async_trait;

#[async_trait]
pub trait FetchRawTemplate {
    async fn fetch_raw_template(&self, id: &str) -> Result<String, reqwest::Error>;
}

#[async_trait]
impl FetchRawTemplate for super::WikiClient {
    async fn fetch_raw_template(&self, id: &str) -> Result<String, reqwest::Error> {
        let template_url = format!("{}/Template:{}?action=raw", &self.base_url, &id);
        let result = self.client.get(template_url).send().await?.text().await;

        result
    }
}
