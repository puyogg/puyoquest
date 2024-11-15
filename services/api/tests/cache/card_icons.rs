use api::{cache::CardIconUrls, config::ApiConfig};
use redis::{AsyncCommands, RedisError};

use crate::common::{create_test_client, seed, IntTestResult};
use api::cache::card_icons;

// #[tokio::test]
// async fn fetches_from_wiki_real() -> IntTestResult {
//     let (_, _, redis_client, wiki_client, s3_client, ..) = create_test_client(
//         "https://puyonexus.com/mediawiki/api.php",
//         "https://puyonexus.com/wiki",
//     )
//     .await?;

//     let card = seed::cards::ALLY_AND_RAFISOL_07.clone();

//     let config = ApiConfig::new().await?;
//     let image_cache_base_url = &config.get_image_cache_domain().await?;
//     let icons = card_icons(
//         &redis_client,
//         &wiki_client,
//         &s3_client,
//         image_cache_base_url.as_str(),
//         &card,
//     )
//     .await?;
//     println!("{:?}", icons);

//     assert_eq!("1", "1");

//     Ok(())
// }

#[tokio::test]
async fn fetches_from_cache() -> IntTestResult {
    let (_, _, redis_client, wiki_client, s3_client, ..) = create_test_client("N/A", "N/A").await?;
    
    let card = seed::cards::ALLY_AND_RAFISOL_07.clone();
    let cached_icon_urls = CardIconUrls {
        normal: Some("https://d14ks6gfutzo56.cloudfront.net/0/06/Img545507.png".to_string()),
        dual_shift: Some("https://d14ks6gfutzo56.cloudfront.net/a/ae/Img545507_msft.png".to_string()),
        extra_power: Some("https://d14ks6gfutzo56.cloudfront.net/a/ab/Img545517.png".to_string()),
        extra_power_dual_shift: Some("https://d14ks6gfutzo56.cloudfront.net/4/48/Img545517_msft.png".to_string())
    };

    let mut redis_conn = redis_client.conn.clone();
    
    let _: Result<String, RedisError> = redis_conn
        .set(
            &redis_client.prefixed(&format!("card_icons:{}", &card.card_id)),
            serde_json::to_string(&cached_icon_urls).unwrap(),
        )
        .await;

    let image_cache_base_url = "N/A";
    let icons = card_icons(
        &redis_client,
        &wiki_client,
        &s3_client,
        image_cache_base_url,
        &card
    ).await?;

    assert_eq!(cached_icon_urls.normal, icons.normal);
    assert_eq!(cached_icon_urls.dual_shift, icons.dual_shift);
    assert_eq!(cached_icon_urls.extra_power, icons.extra_power);
    assert_eq!(cached_icon_urls.extra_power_dual_shift, icons.extra_power_dual_shift);

    Ok(())
}
