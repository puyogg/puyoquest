use crate::cards::template_data::CardTemplateData;
use crate::util::resolve_card_template::resolve_card_template;
use poem::error::InternalServerError;
use poem_openapi::types::ToJSON;
use redis::{AsyncCommands, RedisError};
use serde_json::Value;
use wiki::wiki_client::{FetchTemplate, WikiClient};

use super::RedisClient;

pub async fn card_template_data(
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    card_id: &str,
) -> Result<CardTemplateData, poem::Error> {
    let mut redis_conn = redis_client.conn.clone();
    let key = redis_client.prefixed(&format!("template:{}", &card_id));

    let cached_wiki_template = redis_conn
        .get::<&str, Option<String>>(&key)
        .await
        .inspect_err(|e| println!("{e}"))
        .map_err(InternalServerError)?
        .and_then(|w| {
            let template = serde_json::from_str::<Value>(&w);

            if let Err(e) = &template {
                println!("Error parsing cached wiki_template for: {}", &card_id);
                println!("{}", e);
            }

            template.ok()
        });

    let wiki_template = match cached_wiki_template {
        Some(c) => c.clone(),
        None => {
            let fetched_template = wiki_client
                .fetch_template(&card_id)
                .await
                .map_err(|e| InternalServerError(e))?;

            let fetched_template = serde_json::from_value::<CardTemplateData>(fetched_template)
                .map_err(InternalServerError)?;

            let fetched_template = resolve_card_template(wiki_client, fetched_template)
                .await
                .map_err(InternalServerError)?;

            let fetched_template =
                serde_json::value::to_value(fetched_template).map_err(InternalServerError)?;

            let _: std::result::Result<String, RedisError> = redis_conn
                .set(&key, &fetched_template.to_json_string())
                .await;
            let _ = redis_conn
                .expire::<&str, i64>(
                    &key, 604800, // 7 days
                )
                .await;

            fetched_template
        }
    };

    let wiki_template =
        serde_json::from_value::<CardTemplateData>(wiki_template).map_err(InternalServerError);
    wiki_template
}
