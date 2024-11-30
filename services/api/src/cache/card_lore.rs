use poem::error::InternalServerError;
use redis::AsyncCommands;

use crate::cache::RedisClient;
use crate::cards::lore::WikiLore;
use crate::util::resolve_lore_template::resolve_lore_template;
use wiki::wiki_client::{FetchTemplate, WikiClient};

pub async fn card_lore_data(
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    card_id: &str,
) -> Result<WikiLore, poem::Error> {
    let mut redis_conn = redis_client.conn.clone();
    let key = redis_client.prefixed(&format!("lore:{}", &card_id));

    let cached_lore = redis_conn
        .get::<&str, Option<String>>(&key)
        .await
        .inspect_err(|e| println!("{e}"))
        .map_err(InternalServerError)?
        .and_then(|template_str| {
            let template = serde_json::from_str::<WikiLore>(&template_str);

            if let Err(e) = &template {
                println!("Error parsing cached lore for: {}", &card_id);
                println!("{e}");
            }

            template.ok()
        });

    let lore = match cached_lore {
        Some(l) => l,
        None => {
            let fetched_lore = wiki_client
                .fetch_template(&card_id)
                .await
                .map_err(InternalServerError)?;

            let fetched_lore =
                serde_json::from_value::<WikiLore>(fetched_lore).map_err(InternalServerError)?;

            let fetched_lore = resolve_lore_template(wiki_client, fetched_lore)
                .await
                .map_err(InternalServerError)?;

            let cache_string = serde_json::to_string(&fetched_lore).map_err(InternalServerError)?;
            let set_cache_result = redis_conn
                .set::<&str, String, Option<String>>(&key, cache_string)
                .await
                .inspect_err(|e| {
                    println!("Warning! Failed to cache lore for card_id: {}", &card_id,);
                    println!("{e}");
                });

            if set_cache_result.is_ok() {
                let _ = redis_conn.expire::<&str, i64>(&key, 86400).await; // 1 day
            }

            fetched_lore
        }
    };

    Ok(lore)
}
