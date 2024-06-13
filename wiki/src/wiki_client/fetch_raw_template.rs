use std::future::Future;

pub trait FetchRawTemplate {
    fn fetch_raw_template(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<String, reqwest::Error>> + Send;
}

impl FetchRawTemplate for super::WikiClient {
    async fn fetch_raw_template(&self, id: &str) -> Result<String, reqwest::Error> {
        let template_url = format!("{}/Template:{}?action=raw", &self.base_url, &id);
        let result = self.client.get(template_url).send().await?.text().await;

        result
    }
}

pub async fn fetch_raw_template<T: FetchRawTemplate>(client: &T, id: &str) -> Result<String, reqwest::Error> {
    client.fetch_raw_template(id).await
}
