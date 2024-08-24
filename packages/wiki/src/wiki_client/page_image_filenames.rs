use crate::util::mediawiki_types::{BatchComplete, MediawikiQuery};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PageImageFilenamesContinue {
    pub r#continue: String,
    pub imcontinue: String,
}

#[derive(Deserialize)]
pub struct PageImageFilenamesPageItem {
    pub ns: i32,
    pub title: String,
}

#[derive(Deserialize)]
pub struct PageImageFilenamesPage {
    pub pageid: i32,
    pub ns: i32,
    pub title: String,
    pub images: Vec<PageImageFilenamesPageItem>,
}

#[async_trait]
pub trait PageImageFilenames {
    async fn page_image_filenames(&self, page_title: &str) -> Result<Vec<String>, reqwest::Error>;
}

#[async_trait]
impl PageImageFilenames for super::WikiClient {
    async fn page_image_filenames(&self, page_title: &str) -> Result<Vec<String>, reqwest::Error> {
        let mut filenames: Vec<String> = Vec::new();
        let mut r#continue: Option<String> = None;
        let mut imcontinue: Option<String> = None;

        for _ in 0..50 {
            let mut query_params = vec![
                ("action", "query"),
                ("format", "json"),
                ("prop", "images"),
                ("titles", page_title),
                ("formatversion", "2"),
            ];

            if r#continue.is_some() && imcontinue.is_some() {
                let cont: &str = r#continue.as_ref().unwrap();
                let imcont: &str = imcontinue.as_ref().unwrap();
                query_params.push(("imcontinue", imcont));
                query_params.push(("continue", cont));
            }

            let result = self
                .client
                .get(&self.api_url)
                .query(&query_params)
                .send()
                .await?;

            // println!("{}", result.url());

            let result = result.json::<MediawikiQuery<PageImageFilenamesContinue, PageImageFilenamesPage>>().await?;

            for page in result.query.pages {
                for item in page.images {
                    let image_name = item.title;
                    filenames.push(image_name);
                }
            }

            match result.r#continue {
                Some(c) => {
                    r#continue = Some(c.r#continue);
                    imcontinue = Some(c.imcontinue);
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
        
        Ok(filenames)
    }
}
