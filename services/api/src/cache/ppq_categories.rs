use moka::future::Cache;
use poem::error::InternalServerError;
use std::{sync::LazyLock, time::Duration};
use wiki::wiki_client::{AllCategories, WikiClient};

pub static CATEGORY_CACHE: LazyLock<Cache<String, Vec<String>>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(10)
        .time_to_live(Duration::from_secs(43200))
        .build()
});

const KEY: &'static str = "CATEGORIES";

pub async fn ppq_categories(wiki_client: &WikiClient) -> Result<Vec<String>, poem::Error> {
    let categories = CATEGORY_CACHE.get(KEY).await;
    let categories = match categories {
        Some(c) => c,
        None => {
            println!("Fetching categories from wiki.");
            wiki_client
                .all_ppq_categories()
                .await
                .map_err(InternalServerError)?
                .into_iter()
                .map(|c| c.category.replacen("PPQ:", "", 1))
                .collect()
        }
    };

    Ok(categories)
}
