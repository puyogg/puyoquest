use crate::api_tag::ApiTag;
use poem::{error::InternalServerError, web::Data};
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    ApiResponse, OpenApi,
};
use utils::normalize_name;
use wiki::wiki_client::WikiClient;

pub struct Categories;

#[derive(ApiResponse)]
enum CategoryListResponse {
    #[oai(status = 200)]
    Categories(Json<Vec<String>>),

    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

#[OpenApi(prefix_path = "/categories", tag = "ApiTag::Categories")]
impl Categories {
    #[oai(path = "/", method = "get")]
    async fn list(
        &self,
        wiki_client: Data<&WikiClient>,
        starts_with: Query<String>,
    ) -> Result<CategoryListResponse, poem::Error> {
        let starts_with = normalize_name(&starts_with.0);
        if starts_with.len() == 0 {
            return Ok(CategoryListResponse::BadRequest(PlainText(
                "Invalid starts_with".to_string(),
            )));
        }

        let categories = crate::cache::ppq_categories(&wiki_client.0)
            .await
            .map_err(InternalServerError)?;
        let category_names: Vec<String> = categories
            .into_iter()
            .filter(|c| c.normalized_name.starts_with(&starts_with))
            .map(|c| c.name)
            .collect();

        let category_names = category_names[..std::cmp::min(category_names.len(), 10)].to_vec();

        Ok(CategoryListResponse::Categories(Json(category_names)))
    }
}
