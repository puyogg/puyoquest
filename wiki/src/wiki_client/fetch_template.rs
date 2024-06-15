use async_trait::async_trait;
use serde_json::{Map, Value};
use std::future::Future;

use super::FetchRawTemplate;
use crate::util::{parse_template, ParseTemplateError};

#[derive(Debug)]
pub enum FetchTemplateError {
    Reqwest(reqwest::Error),
    ParseTemplate(ParseTemplateError),
}

impl std::error::Error for FetchTemplateError {}
impl std::fmt::Display for FetchTemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchTemplateError::Reqwest(e) => e.fmt(f),
            FetchTemplateError::ParseTemplate(e) => e.fmt(f),
        }
    }
}

impl From<reqwest::Error> for FetchTemplateError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<ParseTemplateError> for FetchTemplateError {
    fn from(e: ParseTemplateError) -> Self {
        Self::ParseTemplate(e)
    }
}

#[async_trait]
pub trait FetchTemplate {
    // fn fetch_template(
    //     &self,
    //     id: &str,
    // ) -> impl Future<Output = Result<Map<String, Value>, FetchTemplateError>> + Send;
    async fn fetch_template(&self, id: &str) -> Result<Map<String, Value>, FetchTemplateError>;
}

#[async_trait]
impl FetchTemplate for super::WikiClient {
    /// Fetches template and parses it into a serde_json Map.
    async fn fetch_template(&self, id: &str) -> Result<Map<String, Value>, FetchTemplateError> {
        let raw_template = self.fetch_raw_template(id).await?;
        let result = parse_template(&raw_template)?;

        Ok(result)
    }
}
