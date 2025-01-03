use poem::{error::InternalServerError, web::Data};
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    ApiResponse, OpenApi,
};
use sqlx::PgPool;
use utils::normalize_name;
use wiki::wiki_client::WikiClient;

use crate::{aws::s3::S3BackupClient, cache::{self, RedisClient}, config::ApiConfig};

use super::types::Card;

#[derive(ApiResponse)]
enum CategorySearchResponse {
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
        return Ok(CategorySearchResponse::BadRequest(PlainText(format!("Invalid categories: {}", validation.invalid_categories.join(", ")))))
    }
    let categories = validation.valid_categories;

    todo!();
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
        let found_category = all_categories.iter().find(|c| c.normalized_name == normalized_cat);

        match found_category {
            Some(found_category) => result.valid_categories.push(found_category.name.to_string()),
            None => result.invalid_categories.push(cat.to_string()),
        };
    }

    Ok(result)
}
