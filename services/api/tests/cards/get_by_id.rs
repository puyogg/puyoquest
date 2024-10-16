use api::{cards::types::CardCreate, characters::types::CharacterCreate};
use poem::http::StatusCode;
use poem_openapi::types::ToJSON;

use crate::common::{create_test_client, create_test_pool, seed};

#[tokio::test]
async fn gets_by_id2() -> Result<(), Box<dyn std::error::Error>> {
    let (client, test_db_name, ..) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;

    let character = CharacterCreate::from(seed::characters::ARLE.clone());
    let card = CardCreate::from(seed::cards::ARLE_07.clone());

    api::characters::upsert::upsert(&pool, &seed::characters::ARLE.char_id, &character)
        .await
        .unwrap();
    api::cards::upsert::upsert(&pool, &card).await.unwrap();

    let response = client.get("/cards/201207").send().await;
    response.assert_status(StatusCode::OK);
    response.assert_json(card.to_json()).await;

    Ok(())
}
