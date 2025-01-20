use wiki::wiki_client::{FetchMonthlyEvents, WikiClient};

#[tokio::test]
#[ignore]
async fn fetchees_monthly_events_real() -> Result<(), Box<dyn std::error::Error>> {
    let client = WikiClient::new("N/A", "https://puyonexus.com/wiki");

    let events = client.monthly_events().await?;
    println!("{:?}", events);
    assert!(events.len() > 3);

    Ok(())
}
