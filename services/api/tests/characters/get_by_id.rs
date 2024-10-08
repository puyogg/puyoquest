use api::characters::types::CharacterCreate;
use poem::http::StatusCode;

use api::characters::upsert::upsert;
use poem_openapi::types::ToJSON;

use crate::common::{create_test_client, create_test_pool, seed};

#[tokio::test]
async fn gets_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let (client, test_db_name, ..) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;
    let _ = sqlx::query("TRUNCATE character;").execute(&pool).await;

    let character_create = CharacterCreate::from(seed::characters::ARLE.clone());
    let _ = upsert(&pool, &seed::characters::ARLE.char_id, &character_create).await;

    let response = client.get("/characters/2012").send().await;
    response.assert_status(StatusCode::OK);
    response
        .assert_json(seed::characters::ARLE.clone().to_json())
        .await;

    Ok(())
}
