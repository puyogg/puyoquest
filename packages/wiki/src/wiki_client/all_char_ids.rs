use std::sync::LazyLock;

use async_trait::async_trait;
use fancy_regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AllCharIdsResponse {
    pub batchcomplete: bool,
    pub r#continue: Option<AllCharIdsContinue>,
    pub query: AllCharIdsQuery,
}

#[derive(Debug, Deserialize)]
pub struct AllCharIdsContinue {
    pub apcontinue: String,
    pub r#continue: String,
}

#[derive(Debug, Deserialize)]
pub struct AllCharIdsQuery {
    pub allpages: Vec<AllCharIdsItem>,
}

#[derive(Debug, Deserialize)]
pub struct AllCharIdsItem {
    pub pageid: i64,
    pub ns: i64,
    pub title: String,
}

#[async_trait]
pub trait AllCharIds {
    async fn all_char_ids(&self) -> Result<Vec<String>, reqwest::Error>;
}

// https://puyonexus.com/mediawiki/api.php?action=query&format=json&list=allpages&meta=&formatversion=2&apfrom=&apnamespace=10&aplimit=max

pub static RE_TEMPLATE_WITH_CHAR_ID: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^Template:\d\d\d\d$").unwrap());

#[async_trait]
impl AllCharIds for super::WikiClient {
    async fn all_char_ids(&self) -> Result<Vec<String>, reqwest::Error> {
        let mut ids: Vec<String> = Vec::new();
        let mut r#continue: Option<String> = None;
        let mut apcontinue: Option<String> = None;

        for _i in 0..30 {
            let mut query_params = vec![
                ("action", "query"),
                ("format", "json"),
                ("formatversion", "2"),
                ("list", "allpages"),
                ("apnamespace", "10"), // Namespace for "Template" on the PN Wiki
                ("aplimit", "max"),
            ];

            if let (Some(r#continue), Some(apcontinue)) = (&r#continue, &apcontinue) {
                query_params.push(("continue", r#continue));
                query_params.push(("apcontinue", apcontinue));
            }

            let result = self
                .client
                .get(&self.api_url)
                .query(&query_params)
                .send()
                .await?
                .json::<AllCharIdsResponse>()
                .await?;

            for item in result.query.allpages {
                if RE_TEMPLATE_WITH_CHAR_ID
                    .is_match(&item.title)
                    .ok()
                    .unwrap_or(false)
                {
                    ids.push(item.title.replace("Template:", ""));
                }
            }

            match result.r#continue {
                Some(c) => {
                    r#continue = Some(c.r#continue);
                    apcontinue = Some(c.apcontinue);
                }
                None => break,
            };
        }

        Ok(ids)
    }
}
