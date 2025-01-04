use crate::api_tag::ApiTag;
use num::ToPrimitive;
use poem::{error::InternalServerError, web::Data};
use poem_openapi::{param::Query, payload::Json, ApiResponse, OpenApi};
use utils::normalize_name;
use wiki::wiki_client::WikiClient;

pub struct CategoriesRouter;

fn default_limit() -> i32 {
    10
}

fn default_exact() -> bool {
    false
}

#[derive(ApiResponse)]
enum CategoryListResponse {
    #[oai(status = 200)]
    Categories(Json<Vec<String>>),
}

#[OpenApi(prefix_path = "/categories", tag = "ApiTag::Categories")]
impl CategoriesRouter {
    #[oai(path = "/", method = "get")]
    async fn list(
        &self,
        wiki_client: Data<&WikiClient>,
        #[oai(validator(min_length = "1"))] name: Query<String>,
        #[oai(
            default = "default_limit",
            validator(minimum(value = "1"), maximum(value = "10"))
        )]
        limit: Query<i32>,
        #[oai(default = "default_exact")] exact: Query<bool>,
    ) -> Result<CategoryListResponse, poem::Error> {
        let name = normalize_name(&name.0);
        let limit = limit.0.to_usize();
        let exact = exact.0;

        let categories = crate::cache::ppq_categories(&wiki_client.0)
            .await
            .map_err(InternalServerError)?;

        let category_names: Vec<String> = categories
            .into_iter()
            .filter(|c| match exact {
                true => c.normalized_name == name,
                false => c.normalized_name.starts_with(&name),
            })
            .map(|c| c.name)
            .collect();

        let category_names: Vec<String> = match limit {
            None => category_names,
            Some(i) => category_names[..std::cmp::min(category_names.len(), i)].to_vec(),
        };

        Ok(CategoryListResponse::Categories(Json(category_names)))
    }
}
