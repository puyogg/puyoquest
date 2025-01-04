use std::collections::{HashMap, HashSet};

use futures::{future::try_join_all, StreamExt};
use poem::error::InternalServerError;
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse,
};
use sqlx::PgPool;
use utils::{normalize_name, RE_RARITY_SUFFIX};
use wiki::wiki_client::{CategoryMembers, WikiClient};

use crate::{
    aws::s3::S3BackupClient,
    cache::{self, RedisClient},
    config::ApiConfig,
};

use super::types::{Card, CardDb};

#[derive(ApiResponse)]
pub enum CategorySearchResponse {
    #[oai(status = 200)]
    Cards(Json<Vec<Card>>),

    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

pub async fn category_search(
    api_config: &ApiConfig,
    pool: &PgPool,
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    s3_client: &S3BackupClient,
    categories: &Vec<String>,
) -> poem::Result<CategorySearchResponse> {
    let validation = validate_categories(wiki_client, categories).await?;
    if validation.invalid_categories.len() > 0 {
        return Ok(CategorySearchResponse::BadRequest(PlainText(format!(
            "Invalid categories: {}",
            validation.invalid_categories.join(", ")
        ))));
    }
    let categories = validation.valid_categories;

    let category_member_futures = categories.iter().map(|c| wiki_client.category_members(&c));
    let cards_each_category: Vec<Vec<String>> = try_join_all(category_member_futures)
        .await
        .map_err(InternalServerError)?
        .into_iter()
        .map(|link_names| {
            link_names
                .into_iter()
                .filter(|link_name| match RE_RARITY_SUFFIX.is_match(link_name) {
                    Ok(b) => b,
                    Err(e) => {
                        println!("Category member filtering error: {}", e);
                        false
                    }
                })
                .map(|link_name| link_name.replace("PPQ:", ""))
                .collect()
        })
        .collect();
    let cards_each_category: Vec<HashSet<String>> = cards_each_category
        .into_iter()
        .map(|c| HashSet::from_iter(c))
        .collect();

    let mut cats_iter = cards_each_category.iter();
    let intersection = cats_iter.next().map(|set| {
        cats_iter.fold(set.clone(), |set1, set2| {
            set1.intersection(&set2).cloned().collect()
        })
    });

    let intersection = match intersection {
        Some(i) => i,
        None => return Ok(CategorySearchResponse::Cards(Json(vec![]))),
    };

    // Deduplicate based on highest rarity found
    let mut character_and_highest_rarity: HashMap<String, &str> = HashMap::new();
    intersection.iter().for_each(|link_name| {
        let name_no_rarity = RE_RARITY_SUFFIX.replace(link_name, "").to_string();

        let current_link_name = character_and_highest_rarity.get(&name_no_rarity);
        match current_link_name {
            None => {
                character_and_highest_rarity.insert(name_no_rarity, link_name.as_str());
            }
            Some(current_link_name) => {
                let ordering = current_link_name.cmp(&link_name.as_str());
                match ordering {
                    std::cmp::Ordering::Less => {
                        character_and_highest_rarity.insert(name_no_rarity, &link_name);
                    }
                    std::cmp::Ordering::Equal => {
                        character_and_highest_rarity.insert(name_no_rarity, &link_name);
                    }
                    std::cmp::Ordering::Greater => (),
                }
            }
        }
    });
    let link_names: Vec<&str> = character_and_highest_rarity.into_values().collect();

    let card_dbs: Vec<CardDb> = sqlx::query_as(
        r#"
        SELECT *
        FROM card
        WHERE link_name = ANY($1)
    "#,
    )
    .bind(&link_names)
    .fetch_all(pool)
    .await
    .map_err(InternalServerError)?;

    let card_futures = card_dbs.into_iter().map(|card_db| {
        Card::upgrade_card_db(api_config, redis_client, wiki_client, s3_client, card_db)
    });

    let stream = futures::stream::iter(card_futures).buffered(10);
    let mut cards = stream
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .filter_map(|f| f.ok())
        .collect::<Vec<Card>>();
    cards.sort_by(|a, b| a.card_id.cmp(&b.card_id));

    Ok(CategorySearchResponse::Cards(Json(cards)))
}

struct ValidateCategoriesResult {
    valid_categories: Vec<String>,
    invalid_categories: Vec<String>,
}

async fn validate_categories(
    wiki_client: &WikiClient,
    categories: &Vec<String>,
) -> Result<ValidateCategoriesResult, poem::Error> {
    let all_categories = cache::ppq_categories(wiki_client)
        .await
        .map_err(InternalServerError)?;

    let mut result = ValidateCategoriesResult {
        valid_categories: Vec::new(),
        invalid_categories: Vec::new(),
    };

    for cat in categories {
        let normalized_cat = normalize_name(&cat);
        let found_category = all_categories
            .iter()
            .find(|c| c.normalized_name == normalized_cat);

        match found_category {
            Some(found_category) => result
                .valid_categories
                .push(found_category.name.to_string()),
            None => result.invalid_categories.push(cat.to_string()),
        };
    }

    Ok(result)
}
