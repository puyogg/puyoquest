use poem::error::InternalServerError;
use redis::AsyncCommands;
use redis::RedisError;
use wiki::wiki_client::CardAndMaterialIds;
use wiki::wiki_client::CharacterCardIds;
use wiki::wiki_client::WikiClient;

use super::RedisClient;

pub async fn character_card_ids(
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    char_id: &str,
    fetch_fresh: bool,
) -> Result<CardAndMaterialIds, poem::Error> {
    let mut redis_conn = redis_client.conn.clone();
    let key = redis_client.prefixed(&format!("cards_mats:{char_id}"));
    let cached_card_and_material_ids: Option<CardAndMaterialIds> = match fetch_fresh {
        true => None,
        false => redis_conn
            .get::<&str, Option<String>>(&key)
            .await
            .inspect_err(|e| println!("{e}"))
            .map_err(InternalServerError)?
            .and_then(|c| {
                let result = serde_json::from_str::<CardAndMaterialIds>(&c);

                if let Err(e) = &result {
                    println!(
                        "Error parsing cached card and material ids for: {}",
                        &char_id
                    );
                    println!("{e}");
                }

                result.ok()
            }),
    };

    let card_and_material_ids = match cached_card_and_material_ids {
        Some(c) => c,
        None => {
            let card_and_material_ids = wiki_client.character_card_ids(char_id)
                .await
                .map_err(InternalServerError)?;

            let c_string = serde_json::to_string(&card_and_material_ids).map_err(InternalServerError)?;
            let _: Result<String, RedisError> = redis_conn
                .set(&key, &c_string)
                .await;
            let _ = redis_conn
                .expire::<&str, i64>(
                    &key, 86400, // 1 day
                )
                .await;
                
            card_and_material_ids
        }
    };

    Ok(card_and_material_ids)
}
