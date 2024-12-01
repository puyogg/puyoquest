use api::{cards::{full_art::CardFullArtUrls, types::CardCreate}, characters::types::CharacterCreate};
use poem::http::StatusCode;
use redis::AsyncCommands;

use crate::common::{create_test_client, create_test_pool, seed, IntTestResult};

#[tokio::test]
async fn fetches_full_art() -> IntTestResult {
    let (client, test_db_name, redis_client, _, ..) = create_test_client("N/A", "N/A").await?;
    let mut redis_conn = redis_client.conn.clone();
    let key = redis_client.prefixed("full_art:345507");
    
    let pool = create_test_pool(&test_db_name).await?;
    let character = seed::characters::ON_STAGE_RINGO.clone();
    api::characters::upsert::upsert(&pool, &character.char_id.clone(), &CharacterCreate::from(character)).await?;
    let card = seed::cards::ON_STAGE_RINGO_07.clone();
    api::cards::upsert::upsert(&pool, &CardCreate::from(card)).await?;

    let expected_full_art = CardFullArtUrls {
        normal_left: Some("https://d14ks6gfutzo56.cloudfront.net/2/2d/Img345507_l.png".to_string()),
        normal_right: Some("https://d14ks6gfutzo56.cloudfront.net/9/92/Img345507_r.png".to_string()),
        extra_power_left: Some("https://d14ks6gfutzo56.cloudfront.net/c/c2/Img345517_l.png".to_string()),
        extra_power_right: Some("https://d14ks6gfutzo56.cloudfront.net/3/30/Img345517_r.png".to_string()),
        ss: None,
        extra_power_ss: None,
        dual_shift_left: Some("https://d14ks6gfutzo56.cloudfront.net/2/21/Img345507_lsft.png".to_string()),
        dual_shift_right: Some("https://d14ks6gfutzo56.cloudfront.net/d/de/Img345507_rsft.png".to_string()),
        extra_power_dual_shift: Some("https://d14ks6gfutzo56.cloudfront.net/8/80/Img345517_lsft.png".to_string()),
    };
    let cache_string = serde_json::to_string(&expected_full_art).unwrap();
    redis_conn
        .set::<&str, String, Option<String>>(&key, cache_string)
        .await?;

    let response = client
        .get("/cards/345507/full-art")
        .send()
        .await;
    response.assert_status(StatusCode::OK);

    response.assert_json(expected_full_art).await;
    Ok(())
}
