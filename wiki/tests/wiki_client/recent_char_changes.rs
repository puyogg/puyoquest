use std::collections::HashSet;

use chrono::{TimeZone, Utc};
use wiki::wiki_client::{RecentCharChanges, WikiClient};

#[tokio::test]
async fn fetches_recent_changes_real() -> Result<(), Box<dyn std::error::Error>> {
    let client = WikiClient::new("https://puyonexus.com/mediawiki/api.php", "N/A");

    let start = Utc.with_ymd_and_hms(2024, 6, 24, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 6, 27, 0, 0, 0).unwrap();
    let response = client.recent_char_changes(start, end).await?;

    let expected_char_ids: HashSet<String> = vec!["2012"].iter().map(|s| String::from(*s)).collect();

    assert_eq!(response, expected_char_ids);

    Ok(())
}

// #[tokio::test]
// async fn fetches_recent_changes() -> Result<(), Box<dyn std::error::Error>> {
//     let 
// }
