use api::cache::SeriesNameLore;
use api::cards::types::Card;
use poem::http::StatusCode;
use poem_openapi::types::ToJSON;
use redis::AsyncCommands;

use crate::common::seed::seed_arle;
use crate::common::{create_test_client, create_test_pool, seed};

#[tokio::test]
async fn fetches_card_with_template() -> Result<(), Box<dyn std::error::Error>> {
    let (client, test_db_name, redis_client, _, ..) = create_test_client("N/A", "N/A").await?;
    let pool = create_test_pool(&test_db_name).await?;
    seed_arle(&pool).await?;

    let cached_template = seed::cards::ARLE_07.clone().wiki_template;
    let mut redis_conn = redis_client.conn.clone();
    let _: std::result::Result<String, redis::RedisError> = redis_conn
        .set(
            redis_client.prefixed(format!("template:{}", &seed::cards::ARLE_07.card_id).as_str()),
            cached_template.to_json_string(),
        )
        .await;
    let cached_series_data = SeriesNameLore {
        series_name: "Original Puyo Puyo Series".to_string(),
        is_lore: false,
    };
    let _: std::result::Result<String, redis::RedisError> = redis_conn
        .set(
            redis_client.prefixed("char_series:2012"),
            serde_json::to_string(&cached_series_data).unwrap(),
        )
        .await;

    let response = client
        .get("/cards")
        .query("name", &"Arle")
        .query("rarity", &"7")
        .send()
        .await;
    response.assert_status(StatusCode::OK);

    let response_card = response.json().await.value().deserialize::<Card>();
    let expected_card = Card {
        cached_at: response_card.cached_at,
        ..seed::cards::ARLE_07.clone()
    };
    assert_eq!(response_card, expected_card);

    Ok(())
}
