use crate::util::mediawiki_types::{BatchComplete, MediawikiQuery};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CardCategoriesContinue {
    pub clcontinue: String,
    pub r#continue: String,
}

#[derive(Deserialize)]
pub struct CardCategoriesPageItem {
    pub ns: i32,
    pub title: String,
}

#[derive(Deserialize)]
pub struct CardCategoriesPage {
    pub pageid: i32,
    pub ns: i32,
    pub title: String,
    pub categories: Vec<CardCategoriesPageItem>,
}

#[async_trait]
pub trait CardCategories {
    async fn card_categories(
        &self,
        link_name: &str,
    ) -> Result<Vec<String>, reqwest::Error>;
}

#[async_trait]
impl CardCategories for super::WikiClient {
    /// link_name should be formatted like: Altered Satan/★6. DON'T url encode in advance!
    /// Edge cases:
    /// - card page is a redirect because of a different name
    /// things I really don't expect:
    /// - to make sure I don't hammer the API, I limit the pagination requests to 10.
    ///   A character won't be in 100 categories, right?
    async fn card_categories(&self, link_name: &str) -> Result<Vec<String>, reqwest::Error> {
        let mut categories: Vec<String> = Vec::new();
        let mut r#continue: Option<String> = None;
        let mut clcontinue: Option<String> = None;

        for _ in 0..10 {
            let mut query_params = vec![
                ("action", "query"),
                ("format", "json"),
                ("prop", "categories"),
                ("titles", &link_name),
                ("formatversion", "2"),
            ];

            if r#continue.is_some() && clcontinue.is_some() {
                let cont: &str = r#continue.as_ref().unwrap();
                let clcont: &str = clcontinue.as_ref().unwrap();
                query_params.push(("continue", cont));
                query_params.push(("clcontinue", clcont));
            }

            let result = self
                .client
                .get(&self.api_url)
                .query(&query_params)
                .send()
                .await?
                .json::<MediawikiQuery<CardCategoriesContinue, CardCategoriesPage>>()
                .await?;

            for page in result.query.pages {
                for item in page.categories {
                    let clean_category_name = item.title.replace("Category:PPQ:", "").replace("Category:", "");
                    categories.push(clean_category_name);
                }
            }

            match result.r#continue {
                Some(c) => {
                    r#continue = Some(c.r#continue);
                    clcontinue = Some(c.clcontinue);
                },
                None => (),
            };

            match result.batchcomplete {
                Some(batchcomplete) => match batchcomplete {
                    BatchComplete::Boolean(b) => {
                        if b {
                            break;
                        }
                    },
                    BatchComplete::None(_) => (),
                },
                None => (),
            };
        }

        Ok(categories)
    }
}
