use wiki::wiki_client::{AllCategories, WikiClient};

#[tokio::test]
#[ignore]
async fn fetches_all_ppq_categories_real() -> Result<(), Box<dyn std::error::Error>> {
    let client = WikiClient::new("https://puyonexus.com/mediawiki/api.php", "N/A");

    let result = client.all_ppq_categories().await?;
    result.len();

    Ok(())
}
