use api::characters::types::{Character, CharacterCreate};
use chrono::{TimeZone, Utc};
use poem::http::StatusCode;
use poem_openapi::types::ToJSON;

use crate::common::{create_test_client, seed};

#[tokio::test]
async fn inserts_new_character() -> Result<(), Box<dyn std::error::Error>> {
    let (client, _) = create_test_client("N/A", "N/A").await?;

    let character_create = CharacterCreate::from(seed::characters::ARLE.clone());
    let response = client
        .put(format!("/characters/{}", &seed::characters::ARLE.char_id))
        .body_json(&character_create)
        .send()
        .await;

    response.assert_status(StatusCode::OK);
    response.assert_header("Location", "/characters/2012");
    response
        .assert_json(seed::characters::ARLE.clone().to_json())
        .await;

    Ok(())
}

#[tokio::test]
async fn updates_existing_character() -> Result<(), Box<dyn std::error::Error>> {
    let (client, _) = create_test_client("N/A", "N/A").await?;

    let original_character = CharacterCreate::from(seed::characters::ARLE.clone());

    let response1 = client
        .put(format!("/characters/{}", &seed::characters::ARLE.char_id))
        .body_json(&original_character)
        .send()
        .await;
    response1.assert_status(StatusCode::OK);

    let updated_character = Character {
        name: Some("SONIC THE HEDGEHOG!!!!!!!!!".to_string()),
        updated_at: Utc.with_ymd_and_hms(2024, 6, 6, 6, 6, 6).unwrap(),
        ..seed::characters::ARLE.clone()
    };
    let response2 = client
        .put(format!("/characters/{}", &seed::characters::ARLE.char_id))
        .body_json(&CharacterCreate::from(updated_character.clone()))
        .send()
        .await;
    response2.assert_status(StatusCode::OK);
    response2.assert_header("Location", "/characters/2012");
    response2.assert_json(updated_character.to_json()).await;

    Ok(())
}

#[tokio::test]
async fn increases_updated_at_timestamp() -> Result<(), Box<dyn std::error::Error>> {
    let (client, _) = create_test_client("N/A", "N/A").await?;

    let original_character = Character {
        updated_at: Utc.with_ymd_and_hms(1991, 10, 25, 2, 24, 24).unwrap(),
        ..seed::characters::ARLE.clone()
    };

    let mut response1 = client
        .put(format!("/characters/{}", &seed::characters::ARLE.char_id))
        .body_json(&CharacterCreate::from(original_character.clone()))
        .send()
        .await;
    response1.assert_status(StatusCode::OK);

    let update_character_params = CharacterCreate {
        name: Some("SONIC THE HEDGEHOG!!!!!!!!!".to_string()),
        updated_at: None,
        ..CharacterCreate::from(original_character.clone())
    };

    let mut response2 = client
        .put(format!("/characters/{}", &seed::characters::ARLE.char_id))
        .body_json(&update_character_params)
        .send()
        .await;
    response2.assert_status(StatusCode::OK);

    let res1_char: Character = response1.0.take_body().into_json().await.unwrap();
    let res2_char: Character = response2.0.take_body().into_json().await.unwrap();

    assert!(res1_char.updated_at < res2_char.updated_at);

    Ok(())
}
