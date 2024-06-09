use poem::{web::Data, Result};
use poem_openapi::{param::Path, payload::Json, OpenApi};
use sqlx::PgPool;
use crate::api_tag::ApiTag;

pub mod get_by_id;
use get_by_id::GetByIdResponse;
use get_by_id::get_by_id;

pub mod upsert;
use upsert::UpsertResponse;
use upsert::upsert;

pub mod types;
use types::{Character, CharacterCreate};

pub struct CharactersRoute;

#[OpenApi(prefix_path = "/characters", tag = "ApiTag::Characters" )]
impl CharactersRoute {
    /// TODO: Option to refresh index
    #[oai(path = "/:id", method = "get")]
    async fn get_by_id(&self, pool: Data<&PgPool>, id: Path<String>) -> Result<GetByIdResponse> {
        get_by_id(pool, id).await
    }

    /// Create a character or update one if it already exists
    #[oai(path = "/:id", method = "put")]
    async fn upsert(&self, pool: Data<&PgPool>, id: Path<String>, character: Json<CharacterCreate>) -> Result<UpsertResponse> {
        // TODO: Only allow admins to upsert characters
        upsert(pool.0, &id.0, &character.0).await
    }

    // /// Find by [name + rarity] or by [category]
    // #[oai(path = "/", method = "get")]
    // async fn find(&self) -> Result<()> {
    //     todo!();
    // }

    // #[oai(path = "/:id/cards", method = "get")]
    // async fn cards(&self) -> Result<()> {
    //     todo!();
    // }
}
