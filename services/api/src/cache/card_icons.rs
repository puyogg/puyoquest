use std::collections::HashSet;
use std::iter::FromIterator;

use poem::error::InternalServerError;
use poem_openapi::Object;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use wiki::wiki_client::{ImageUrl, PageImageFilenames, WikiClient};

use crate::aws::s3::S3BackupClient;
use crate::cards::types::CardDb;
use crate::util::format_card_link_name::format_card_link_name;

use super::RedisClient;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Object)]
pub struct CardIconUrls {
    pub normal: Option<String>,
    pub dual_shift: Option<String>,
    pub extra_power: Option<String>,
    pub extra_power_dual_shift: Option<String>,
}

impl Default for CardIconUrls {
    fn default() -> Self {
        Self {
            normal: None,
            dual_shift: None,
            extra_power: None,
            extra_power_dual_shift: None,
        }
    }
}

pub async fn card_icons(
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    s3_client: &S3BackupClient,
    image_base_url: &str,
    card: &CardDb,
) -> Result<CardIconUrls, poem::Error> {
    let CardDb {
        char_id,
        card_id,
        link_name,
        rarity,
        rarity_modifier,
        ..
    } = card;
    let mut redis_conn = redis_client.conn.clone();
    let key = redis_client.prefixed(&format!("card_icons:{}", card_id));

    let cached_card_icon_urls = redis_conn
        .get::<&str, Option<String>>(&key)
        .await
        .inspect_err(|e| println!("{e}"))
        .map_err(InternalServerError)?
        .and_then(|json_string| {
            let card_icon_urls = serde_json::from_str::<CardIconUrls>(&json_string);
            match card_icon_urls {
                Err(e) => {
                    println!("Error parsing cached card_icon_urls for: {}", card_id);
                    println!("{}", e);
                    None
                }
                Ok(c) => Some(c),
            }
        });

    match cached_card_icon_urls {
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

            let (normal, dual_shift, extra_power, extra_power_dual_shift) = futures::try_join!(
                backup_pn_image(
                    wiki_client,
                    s3_client,
                    image_base_url,
                    &page_names,
                    format!("File:Img{card_id}.png")
                ),
                backup_pn_image(
                    wiki_client,
                    s3_client,
                    image_base_url,
                    &page_names,
                    format!("File:Img{card_id} msft.png")
                ),
                backup_pn_image(
                    wiki_client,
                    s3_client,
                    image_base_url,
                    &page_names,
                    format!("File:Img{char_id}17.png")
                ),
                backup_pn_image(
                    wiki_client,
                    s3_client,
                    image_base_url,
                    &page_names,
                    format!("File:Img{char_id}17 msft.png")
                ),
            )?;

            let card_icon_urls = CardIconUrls {
                normal,
                dual_shift,
                extra_power,
                extra_power_dual_shift,
            };
            let cache_string =
                serde_json::to_string(&card_icon_urls).map_err(InternalServerError)?;

            let set_cache_result: Option<String> = redis_conn
                .set(&key, cache_string)
                .await
                .inspect_err(|e| {
                    println!(
                        "Warning! Failed to save icon urls to cache for card_id: {}",
                        &card_id
                    );
                    println!("{e}")
                })
                .ok();
            if set_cache_result.is_some() {
                let _ = redis_conn.expire::<&str, i64>(&key, 86400).await; // 1 day
            }

            Ok(card_icon_urls)
        }
    }
}

async fn backup_pn_image(
    wiki_client: &WikiClient,
    s3_client: &S3BackupClient,
    image_base_url: &str,
    page_names: &HashSet<String>,
    file_page_name: String,
) -> Result<Option<String>, poem::Error> {
    if !page_names.contains(&file_page_name) {
        return Ok(None);
    }

    let full_image_pn_url = wiki_client
        .image_url(file_page_name)
        .await
        .map_err(InternalServerError)?;
    let full_image_pn_url = match full_image_pn_url {
        None => return Ok(None),
        Some(u) => u,
    };
    let image_path = full_image_pn_url.replace("https://puyonexus.com/mediawiki/images/", "");

    s3_client
        .backup_image_from_url(&image_path, &full_image_pn_url)
        .await?;

    Ok(Some(format!("{image_base_url}/{image_path}")))
}
