use crate::api_tag::ApiTag;
use poem::web::Data;
use poem_openapi::{param::Path, payload::Json, OpenApi};
use sqlx::PgPool;

pub mod template_data;

pub mod types;
use types::{Card, CardCreate};

pub mod get_by_id;
use get_by_id::{get_by_id, GetByIdResponse};

pub mod upsert;
use upsert::{upsert, UpsertResponse};

pub struct CardsRouter;

#[OpenApi(prefix_path = "/cards", tag = "ApiTag::Cards")]
impl CardsRouter {
    /// Find by card_id
    #[oai(path = "/:id", method = "get")]
    async fn get(&self, pool: Data<&PgPool>, id: Path<String>) -> poem::Result<GetByIdResponse> {
        get_by_id(pool, id).await
    }

    // /// Find by name and rarity
    // /// OR find by category
    // #[oai(path = "/", method = "get")]
    // async fn find(&self) -> Result<()> {
    //     todo!();
    // }

    /// Upsert card data (admins only)
    #[oai(path = "/", method = "post")]
    async fn upsert(
        &self,
        pool: Data<&PgPool>,
        card: Json<CardCreate>,
    ) -> poem::Result<UpsertResponse> {
        // TODO: Apply NFKD normalization before saving
        upsert(pool.0, &card.0).await
    }

    // /// List random cards
    // /// TODO: Set cache control
    // #[oai(path = "/random", method = "get")]
    // async fn random(&self) -> Result<()> {
    //     todo!();
    // }
}
