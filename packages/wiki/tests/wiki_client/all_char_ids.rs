use wiki::wiki_client::{AllCharIds, WikiClient};

#[tokio::test]
#[ignore]
async fn fetches_all_char_ids() -> Result<(), Box<dyn std::error::Error>> {
    let client = WikiClient::new("https://puyonexus.com/mediawiki/api.php", "N/A");

    let char_ids = client.all_char_ids().await?;
    println!("{:?}", char_ids.len());

    Ok(())
}
