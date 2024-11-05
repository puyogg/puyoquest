// use api::aws::s3::S3Client;

// #[tokio::test]
// async fn backup_image_from_url() -> Result<(), Box<dyn std::error::Error>> {
//     let config = aws_config::from_env().load().await;
    
//     let client = S3Client {
//         client: aws_sdk_s3::Client::new(&config),
//         reqwest_client: reqwest::Client::new(),
//         image_cache_bucket_name: "api-pn-image-cache".to_string(),
//     };

//     client.backup_image_from_url("test/test2.png", "https://puyonexus.com/mediawiki/images/a/ad/Img201207.png").await?;
    
//     Ok(())
// }
