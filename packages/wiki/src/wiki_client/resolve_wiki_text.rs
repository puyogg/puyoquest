use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ResolveWikiTextResponseCategories {
    pub sortkey: String,
    pub category: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResolveWikiTextResponseParse {
    pub categories: Vec<ResolveWikiTextResponseCategories>,
    pub images: Vec<String>,
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResolveWikiTextResponse {
    pub parse: ResolveWikiTextResponseParse,
}

#[derive(Debug)]
pub enum ResolveWikiTextError {
    Reqwest(reqwest::Error),
}

impl std::error::Error for ResolveWikiTextError {}
impl std::fmt::Display for ResolveWikiTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolveWikiTextError::Reqwest(error) => error.fmt(f),
        }
    }
}

impl From<reqwest::Error> for ResolveWikiTextError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

#[async_trait]
pub trait ResolveWikiText {
    async fn resolve_wiki_text(
        &self,
        text: &str,
    ) -> Result<ResolveWikiTextResponse, ResolveWikiTextError>;
}

#[async_trait]
impl ResolveWikiText for super::WikiClient {
    async fn resolve_wiki_text(
        &self,
        text: &str,
    ) -> Result<ResolveWikiTextResponse, ResolveWikiTextError> {
        let result = self
            .client
            .get(&self.api_url)
            .query(&[
                ("action", "parse"),
                ("format", "json"),
                ("formatversion", "2"),
                ("contentmodel", "wikitext"),
                ("text", text),
            ])
            .send()
            .await?
            .json::<ResolveWikiTextResponse>()
            .await?;

        Ok(result)
    }
}
