use api::config::ApiConfig;

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
