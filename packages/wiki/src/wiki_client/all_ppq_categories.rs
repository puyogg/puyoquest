use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AllCategoriesResponse {
    pub batchcomplete: bool,
    pub r#continue: Option<AllCategoriesContinue>,
    pub query: AllCategoriesQuery,
}

#[derive(Deserialize)]
pub struct AllCategoriesContinue {
    pub accontinue: String,
    pub r#continue: String,
}

#[derive(Deserialize)]
pub struct AllCategoriesQuery {
    pub allcategories: Vec<AllCategoriesItem>,
}

#[derive(Deserialize)]
pub struct AllCategoriesItem {
    pub category: String,
    pub size: i64,
    pub pages: i64,
    pub files: i64,
    pub subcats: i64,
    pub hidden: bool,
}

#[async_trait]
pub trait AllCategories {
    async fn all_ppq_categories(&self) -> Result<Vec<AllCategoriesItem>, reqwest::Error>;
}

#[async_trait]
impl AllCategories for super::WikiClient {
    async fn all_ppq_categories(&self) -> Result<Vec<AllCategoriesItem>, reqwest::Error> {
        let mut items: Vec<AllCategoriesItem> = Vec::new();
        let mut r#continue: Option<String> = None;
        let mut accontinue: Option<String> = None;

        for i in 0..30 {
            println!("Page: {i}");
            let mut query_params = vec![
                ("action", "query"),
                ("format", "json"),
                ("formatversion", "2"),
                ("list", "allcategories"),
                ("acmin", "1"),
                ("aclimit", "500"),
                ("acprefix", "PPQ:"),
                ("acprop", "size|hidden"),
            ];

            if let (Some(r#continue), Some(accontinue)) = (&r#continue, &accontinue) {
                query_params.push(("continue", r#continue));
                query_params.push(("accontinue", accontinue));
            }

            let mut result = self
                .client
                .get(&self.api_url)
                .query(&query_params)
                .send()
                .await?
                .json::<AllCategoriesResponse>()
                .await?;

            items.append(&mut result.query.allcategories);

            match result.r#continue {
                Some(c) => {
                    r#continue = Some(c.r#continue);
                    accontinue = Some(c.accontinue);
                }
                None => break,
            };
        }

        Ok(items)
    }
}
