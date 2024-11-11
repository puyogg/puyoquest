use std::sync::Arc;

use poem::error::InternalServerError;
use redis::{AsyncCommands, RedisError};
use serde::{Deserialize, Serialize};
use wiki::wiki_client::{FetchCharacterSeries, WikiClient};

use crate::RedisClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesNameLore {
    pub series_name: String,
    pub is_lore: bool,
}

enum CacheResponse {
    SeriesResult((String, bool)),
    IntentionallyBlank,
    KeyNotFound,
}

pub async fn character_series_data(
    redis_client: &Arc<RedisClient>,
    wiki_client: &WikiClient,
    char_id: &str,
    link_name: &str,
) -> Result<Option<(String, bool)>, poem::Error> {
    let mut redis_conn = redis_client.conn.clone();
    let key = redis_client.prefixed(&format!("char_series:{}", char_id));

    let cached_series_result = redis_conn
        .get::<&str, Option<String>>(&key)
        .await
        .inspect_err(|e| println!("{e}"))
        .map_err(InternalServerError)?
        .and_then(|s| {
            if s.len() == 0 {
                return Some(CacheResponse::IntentionallyBlank);
            }

            let series_result = serde_json::from_str::<SeriesNameLore>(&s).map(
                |SeriesNameLore {
                     series_name,
                     is_lore,
                 }| { (series_name, is_lore) },
            );

            if let Err(e) = &series_result {
                println!("Error parsing cached series data for char_id: {}", &char_id);
                println!("{}", e);
            }

            match series_result {
                Err(e) => {
                    println!("Error parsing cached series data for char_id: {}", &char_id);
                    println!("{}", e);
                    None
                },
                Ok(r) => Some(CacheResponse::SeriesResult(r))
            }
        })
        .unwrap_or(CacheResponse::KeyNotFound);

    match cached_series_result {
        CacheResponse::IntentionallyBlank => Ok(None),
        CacheResponse::SeriesResult(s) => Ok(Some(s)),
        CacheResponse::KeyNotFound => {
            let fetched = wiki_client
                .fetch_character_series(char_id, link_name)
                .await
                .map_err(InternalServerError)?;

            let cache_string = match &fetched {
                None => "".to_string(),
                Some(f) => {
                    let obj = SeriesNameLore {
                        series_name: f.0.clone(),
                        is_lore: f.1.clone(),
                    };
                    serde_json::to_string(&obj).map_err(InternalServerError)?
                }
            };

            let _: Result<String, RedisError> = redis_conn.set(&key, cache_string).await;
            let _ = redis_conn
                .expire::<&str, i64>(
                    &key, 604800, // 7 days
                )
                .await;

            Ok(fetched)
        },
    }
}
