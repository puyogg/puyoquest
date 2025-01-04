use moka::future::Cache;
use poem::error::InternalServerError;
use serde::Serialize;
use std::{sync::LazyLock, time::Duration};
use utils::normalize_name;
use wiki::wiki_client::{AllCategories, WikiClient};

#[derive(Debug, Clone, Serialize)]
pub struct Category {
    pub name: String,
    pub normalized_name: String,
}

pub static CATEGORY_CACHE: LazyLock<Cache<&str, Vec<Category>>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(10)
        .time_to_live(Duration::from_secs(43200))
        .build()
});

const KEY: &'static str = "CATEGORIES";

pub async fn ppq_categories(wiki_client: &WikiClient) -> Result<Vec<Category>, poem::Error> {
    let categories = CATEGORY_CACHE.get(KEY).await;
    let categories = match categories {
        Some(c) => c,
        None => {
            let categories: Vec<Category> = wiki_client
                .all_ppq_categories()
                .await
                .map_err(InternalServerError)?
                .into_iter()
                .map(|c| c.category.replacen("PPQ:", "", 1))
                .map(|name| Category {
                    normalized_name: normalize_name(&name),
                    name,
                })
                .collect();

            CATEGORY_CACHE.insert(KEY, categories.clone()).await;

            categories
        }
    };

    Ok(categories)
}
