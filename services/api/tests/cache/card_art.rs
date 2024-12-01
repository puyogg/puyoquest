use api::cards::full_art::CardFullArtUrls;
use redis::AsyncCommands;
use api::cache::card_art;
use api::cards::types::{CardCreate, CardDb};
use api::config::ApiConfig;

use crate::common::{create_test_client, seed, IntTestResult};

#[tokio::test]
#[ignore]
async fn fetches_from_wiki_dual_shift_real() -> IntTestResult {
    let (_, _, redis_client, wiki_client, s3_client, ..) = create_test_client(
        "https://puyonexus.com/mediawiki/api.php",
        "https://puyonexus.com/wiki",
    )
    .await?;
    let mut redis_conn = redis_client.conn.clone();

    let card = seed::cards::ON_STAGE_RINGO_07.clone();
    let card_db = CardDb::from(CardCreate::from(card));
    let key = redis_client.prefixed(&format!("full_art:{}", &card_db.card_id));

    let current_cache = redis_conn
        .get::<&str, Option<String>>(&key)
        .await
        .ok()
        .flatten();
    assert!(current_cache.is_none());

    let config = ApiConfig::new().await?;
    let image_cache_base_url = &config.get_image_cache_domain().await?;

    let full_art_urls = card_art(
        &redis_client,
        &wiki_client,
        &s3_client,
        image_cache_base_url,
        &card_db,
    ).await?;

    assert_eq!(
        full_art_urls,
        CardFullArtUrls {
            normal_left: Some("https://d14ks6gfutzo56.cloudfront.net/2/2d/Img345507_l.png".to_string()),
            normal_right: Some("https://d14ks6gfutzo56.cloudfront.net/9/92/Img345507_r.png".to_string()),
            extra_power_left: Some("https://d14ks6gfutzo56.cloudfront.net/c/c2/Img345517_l.png".to_string()),
            extra_power_right: Some("https://d14ks6gfutzo56.cloudfront.net/3/30/Img345517_r.png".to_string()),
            ss: None,
            extra_power_ss: None,
            dual_shift_left: Some("https://d14ks6gfutzo56.cloudfront.net/2/21/Img345507_lsft.png".to_string()),
            dual_shift_right: Some("https://d14ks6gfutzo56.cloudfront.net/d/de/Img345507_rsft.png".to_string()),
            extra_power_dual_shift: Some("https://d14ks6gfutzo56.cloudfront.net/8/80/Img345517_lsft.png".to_string()),
        }
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn fetches_from_wiki_cross_ability_real() -> IntTestResult {
    let (_, _, redis_client, wiki_client, s3_client, ..) = create_test_client(
        "https://puyonexus.com/mediawiki/api.php",
        "https://puyonexus.com/wiki",
    )
    .await?;
    let mut redis_conn = redis_client.conn.clone();

    let card = seed::cards::RAINCLOUD_NINE_SIG_07.clone();
    let card_db = CardDb::from(CardCreate::from(card));
    let key = redis_client.prefixed(&format!("full_art:{}", &card_db.card_id));

    let current_cache = redis_conn
        .get::<&str, Option<String>>(&key)
        .await
        .ok()
        .flatten();
    assert!(current_cache.is_none());

    let config = ApiConfig::new().await?;
    let image_cache_base_url = &config.get_image_cache_domain().await?;

    let full_art_urls = card_art(
        &redis_client,
        &wiki_client,
        &s3_client,
        image_cache_base_url,
        &card_db,
    ).await?;

    assert_eq!(
        full_art_urls,
        CardFullArtUrls {
            normal_left: Some("https://d14ks6gfutzo56.cloudfront.net/8/8b/Img241207_l.png".to_string()),
            normal_right: Some("https://d14ks6gfutzo56.cloudfront.net/f/f4/Img241207_r.png".to_string()),
            extra_power_left: Some("https://d14ks6gfutzo56.cloudfront.net/c/c8/Img241217_l.png".to_string()),
            extra_power_right: Some("https://d14ks6gfutzo56.cloudfront.net/1/18/Img241217_r.png".to_string()),
            ss: Some("https://d14ks6gfutzo56.cloudfront.net/6/67/Img241207_ss.png".to_string()),
            extra_power_ss: Some("https://d14ks6gfutzo56.cloudfront.net/0/01/Img241217_ss.png".to_string()),
            dual_shift_left: None,
            dual_shift_right: None,
            extra_power_dual_shift: None
        },
    );

    Ok(())
}
