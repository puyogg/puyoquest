use serde_json::json;

pub struct WebhookLogger {
    client: reqwest::Client,
    webhook_url: String,
}

impl WebhookLogger {
    pub fn new(webhook_url: &str) -> WebhookLogger {
        WebhookLogger {
            client: reqwest::Client::new(),
            webhook_url: webhook_url.to_string(),
        }
    }

    pub async fn log(&self, message: &str) -> Result<(), anyhow::Error> {
        let body = json!({
            "content": message,
        });

        self.client
            .post(&self.webhook_url)
            .json(&body)
            .send()
            .await?;

        Ok(())
    }
}
