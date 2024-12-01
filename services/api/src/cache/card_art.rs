use std::collections::HashSet;
use std::iter::FromIterator;

use poem::error::InternalServerError;
use poem_openapi::Object;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use wiki::wiki_client::{ImageUrl, PageImageFilenames, WikiClient};

use crate::aws::s3::S3BackupClient;
use crate::cards::full_art::CardFullArtUrls;
use crate::cards::types::CardDb;
use crate::util::format_card_link_name::format_card_link_name;

use super::RedisClient;
use super::card_icons::backup_pn_image;

pub async fn card_art(
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    s3_client: &S3BackupClient,
    image_base_url: &str,
    card: &CardDb,
) -> Result<CardFullArtUrls, poem::Error> {
    let CardDb {
        char_id,
        card_id,
        link_name,
        rarity,
        rarity_modifier,
        ..
    } = card;
    let mut redis_conn = redis_client.conn.clone();
    let key = redis_client.prefixed(&format!("full_art:{}", card_id));

    let cached_full_art_urls = redis_conn
        .get::<&str, Option<String>>(&key)
        .await
        .inspect_err(|e| println!("{e}"))
        .map_err(InternalServerError)?
        .and_then(|json_string| {
            let full_art_urls = serde_json::from_str::<CardFullArtUrls>(&json_string);

            match full_art_urls {
                Err(e) => {
                    println!("Error parsing cached full art urls for: {}", card_id);
                    println!("{}", e);
                    None
                },
                Ok(c) => Some(c),
            }
        });

    match cached_full_art_urls {
        Some(c) => Ok(c),
        None => {
            let card_link_name = format!(
                "PPQ:{}",
                format_card_link_name(link_name, rarity, rarity_modifier)
            );
            let page_names = wiki_client
                .page_image_filenames(&card_link_name)
                .await
                .map_err(InternalServerError)?;
            let page_names: HashSet<String> = HashSet::from_iter(page_names);

            let backup_image = |file_page_name: String| async {
                backup_pn_image(
                    wiki_client,
                    s3_client,
                    image_base_url,
                    &page_names,
                    file_page_name,
                ).await
            };

            let (
                normal_left,
                normal_right,
                extra_power_left,
                extra_power_right,
                ss,
                extra_power_ss,
                dual_shift_left,
                dual_shift_right,
                extra_power_dual_shift,
            ) = futures::try_join!(
                backup_image(format!("File:Img{card_id} l.png")),
                backup_image(format!("File:Img{card_id} r.png")),
                backup_image(format!("File:Img{char_id}1{rarity} l.png")),
                backup_image(format!("File:Img{char_id}1{rarity} r.png")),
                backup_image(format!("File:Img{card_id} ss.png")),
                backup_image(format!("File:Img{char_id}1{rarity} ss.png")),
                backup_image(format!("File:Img{card_id} lsft.png")),
                backup_image(format!("File:Img{card_id} rsft.png")),
                backup_image(format!("File:Img{char_id}1{rarity} lsft.png"))
            )?;

            let full_art_urls = CardFullArtUrls {
                normal_left,
                normal_right,
                extra_power_left,
                extra_power_right,
                ss,
                extra_power_ss,
                dual_shift_left,
                dual_shift_right,
                extra_power_dual_shift,
            };

            let cache_string = serde_json::to_string(&full_art_urls).map_err(InternalServerError)?;
            let set_cache_result = redis_conn
                .set::<&str, String, Option<String>>(&key, cache_string)
                .await
                .inspect_err(|e| {
                    println!(
                        "Warning! Failed to save full art urls to cache for card_id: {}",
                        &card_id
                    );
                    println!("{e}")
                });

            if set_cache_result.is_ok() {
                let _ = redis_conn.expire::<&str, i64>(&key, 86400).await; // 1 day
            }

            Ok(full_art_urls)
        }
    }
}
