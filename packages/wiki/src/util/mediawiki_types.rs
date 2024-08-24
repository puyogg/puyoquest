use serde::{Deserialize, Serialize};

/// https://www.mediawiki.org/wiki/API:Query
/// - C: the continue shape
/// - P: the page content shape
#[derive(Deserialize)]
pub struct MediawikiQuery<C, P> {
    pub batchcomplete: Option<BatchComplete>,
    pub r#continue: Option<C>,
    pub query: QueryField<P>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BatchComplete {
    Boolean(bool),
    None(String),
}

#[derive(Deserialize)]
pub struct QueryField<P> {
    pub pages: Vec<P>,
}
