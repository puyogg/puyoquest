use api::{cards::types::CardCreate, characters::types::CharacterCreate, init_api};
use poem::http::StatusCode;
use poem::test::TestClient;

use poem_openapi::types::ToJSON;

use crate::common::{create_test_pool, request_test_db, seed};

#[tokio::test]
async fn gets_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let test_db_name = request_test_db().await?;

    let pool = create_test_pool(&test_db_name).await?;

    let character = CharacterCreate::from(seed::characters::ARLE.clone());
    let card = CardCreate::from(seed::cards::ARLE_07.clone());

    let _ = api::characters::upsert::upsert(&pool, &seed::characters::ARLE.char_id, &character).await;
    let _ = api::cards::upsert::upsert(&pool,  &card).await;

    let api = init_api(pool);
    let client = TestClient::new(api);

    let response = client.get("/cards/201207").send().await;
    response.assert_status(StatusCode::OK);
    response.assert_json(card.to_json()).await;

    Ok(())
}
