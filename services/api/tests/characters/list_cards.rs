use api::cards::types::Card;
use api::characters::list_cards::CardsAndMaterials;
use chrono::{TimeZone, Utc};
use poem::http::StatusCode;
use redis::AsyncCommands;
use wiki::wiki_client::CardAndMaterialIds;

use crate::common::seed::{self, seed_santa_ringo};
use crate::common::{create_test_client, create_test_pool, IntTestResult};

#[tokio::test]
async fn lists_cards_by_char_id() -> IntTestResult {
    let (client, test_db_name, redis_client, wiki_client, s3_client, ..) =
        create_test_client("N/A", "N/A").await?;
    let mut redis_conn = redis_client.conn.clone();
    let pool = create_test_pool(&test_db_name).await?;
    seed_santa_ringo(&pool).await?;

    let cached_id_data = CardAndMaterialIds {
        card_ids: vec!["321204", "321205", "321206", "321216"]
            .into_iter()
            .map(|v| v.to_string())
            .collect(),
        material_ids: vec!["352905", "352906", "152806"]
            .into_iter()
            .map(|v| v.to_string())
            .collect(),
    };
    let cached_id_data_string = serde_json::to_string(&cached_id_data).unwrap();
    redis_conn
        .set::<String, &str, Option<String>>(
            redis_client.prefixed("cards_mats:3212"),
            &cached_id_data_string,
        )
        .await
        .unwrap();
    redis_conn
        .set::<String, &str, Option<String>>(redis_client.prefixed("char_series:3212"), "")
        .await
        .unwrap();

    let card_icon_data = [
        ("321204", seed::cards::SANTA_RINGO_04.clone()),
        ("321205", seed::cards::SANTA_RINGO_05.clone()),
        ("321206", seed::cards::SANTA_RINGO_06.clone()),
        ("321216", seed::cards::SANTA_RINGO_6S.clone()),
        ("352905", seed::cards::SANTA_RINGO_MAT1.clone()),
        ("352906", seed::cards::SANTA_RINGO_MAT2.clone()),
        ("152806", seed::cards::SANTA_RINGO_MAT3.clone()),
    ];
    for (card_id, card) in card_icon_data {
        redis_conn
            .set::<String, String, Option<String>>(
                redis_client.prefixed(&format!("template:{card_id}")),
                serde_json::to_string(&card.wiki_template).unwrap(),
            )
            .await?;

        let card_icons = &card.icons;
        redis_conn
            .set::<String, String, Option<String>>(
                redis_client.prefixed(&format!("card_icons:{card_id}")),
                serde_json::to_string(card_icons).unwrap(),
            )
            .await?;
    }

    let response = client.get("/characters/3212/cards").send().await;
    response.assert_status(StatusCode::OK);

    let expected_response = CardsAndMaterials {
        cards: vec![
            seed::cards::SANTA_RINGO_04.clone(),
            seed::cards::SANTA_RINGO_05.clone(),
            seed::cards::SANTA_RINGO_06.clone(),
            seed::cards::SANTA_RINGO_6S.clone(),
        ],
        materials: vec![
            seed::cards::SANTA_RINGO_MAT1.clone(),
            seed::cards::SANTA_RINGO_MAT2.clone(),
            seed::cards::SANTA_RINGO_MAT3.clone(),
        ],
    };

    // Need to force cached_at values to match
    let response = response
        .json()
        .await
        .value()
        .deserialize::<CardsAndMaterials>();
    let response = CardsAndMaterials {
        cards: response
            .cards
            .into_iter()
            .map(|c| Card {
                cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
                ..c.clone()
            })
            .collect(),
        materials: response
            .materials
            .into_iter()
            .map(|c| Card {
                cached_at: Utc.with_ymd_and_hms(2024, 2, 24, 14, 24, 24).unwrap(),
                ..c.clone()
            })
            .collect(),
    };

    assert_eq!(response, expected_response);

    Ok(())
}
