use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CategoryMembersResponse {
    pub batchcomplete: bool,
    pub r#continue: Option<CategoryMembersContinue>,
    pub query: CategoryMembersQuery,
}

#[derive(Debug, Deserialize)]
pub struct CategoryMembersContinue {
    pub cmcontinue: String,
    pub r#continue: String,
}

#[derive(Debug, Deserialize)]
pub struct CategoryMembersQuery {
    pub categorymembers: Vec<CategoryMembersItem>,
}

#[derive(Debug, Deserialize)]
pub struct CategoryMembersItem {
    pub ns: i32,
    pub title: String,
}

#[async_trait]
pub trait CategoryMembers {
    async fn category_members(&self, category: &str) -> Result<Vec<String>, reqwest::Error>;
}

#[async_trait]
impl CategoryMembers for super::WikiClient {
    async fn category_members(&self, category: &str) -> Result<Vec<String>, reqwest::Error> {
        let mut ppq_link_names: Vec<String> = Vec::new();
        let mut r#continue: Option<String> = None;
        let mut cmcontinue: Option<String> = None;

        for _i in 0..50 {
            let cmtitle = format!("Category:PPQ:{}", category);
            let mut query_params = vec![
                ("action", "query"),
                ("format", "json"),
                ("formatversion", "2"),
                ("list", "categorymembers"),
                ("cmtitle", &cmtitle),
                ("cmlimit", "500"),
                ("cmprop", "title"),
            ];

            if let (Some(r#continue), Some(cmcontinue)) = (&r#continue, &cmcontinue) {
                query_params.push(("continue", r#continue));
                query_params.push(("cmcontinue", cmcontinue));
            }

            let result = self
                .client
                .get(&self.api_url)
                .query(&query_params)
                .send()
                .await?
                .json::<CategoryMembersResponse>()
                .await?;

            // let result_continue = result.r#continue;

            let mut batch_link_names: Vec<String> = result
                .query
                .categorymembers
                .into_iter()
                .map(|c| c.title.clone())
                .collect();
            ppq_link_names.append(&mut batch_link_names);

            match result.r#continue {
                Some(c) => {
                    r#continue = Some(c.r#continue);
                    cmcontinue = Some(c.cmcontinue);
                }
                None => break,
            }
        }

        Ok(ppq_link_names)
    }
}
