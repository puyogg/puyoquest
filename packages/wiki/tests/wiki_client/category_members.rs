use wiki::wiki_client::{CategoryMembers, WikiClient};

#[tokio::test]
#[ignore]
async fn fetches_all_heavenly_knights() -> Result<(), Box<dyn std::error::Error>> {
    let client = WikiClient::new("https://puyonexus.com/mediawiki/api.php", "N/A");

    let result = client
        .category_members("Heavenly Knight Series")
        .await?;
    assert_eq!(
        result,
        vec![
            "PPQ:Mars",
            "PPQ:Mars/★4",
            "PPQ:Mars/★5",
            "PPQ:Mars/★6",
            "PPQ:Mars/★7",
            "PPQ:Yuri",
            "PPQ:Yuri/★4",
            "PPQ:Yuri/★5",
            "PPQ:Yuri/★6",
            "PPQ:Yuri/★7",
            "PPQ:Hartmann",
            "PPQ:Hartmann/★4",
            "PPQ:Hartmann/★5",
            "PPQ:Hartmann/★6",
            "PPQ:Hartmann/★7",
            "PPQ:Emilia",
            "PPQ:Emilia/★4",
            "PPQ:Emilia/★5",
            "PPQ:Emilia/★6",
            "PPQ:Emilia/★7",
            "PPQ:Legamünt",
            "PPQ:Legamünt/★6",
            "PPQ:Legamünt/★7",
            "PPQ:Viola",
            "PPQ:Viola/★4",
            "PPQ:Viola/★5",
            "PPQ:Viola/★6",
            "PPQ:Viola/★7"
        ]
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn fetches_all_red_cards() -> Result<(), Box<dyn std::error::Error>> {
    let client = WikiClient::new("https://puyonexus.com/mediawiki/api.php", "N/A");

    let result = client.category_members("Red Color").await?;
    assert!(result.len() > 2000);

    Ok(())
}
