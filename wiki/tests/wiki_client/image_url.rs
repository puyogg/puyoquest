use lazy_static::lazy_static;
use serde_json::{json, Value};
use wiki::wiki_client::{ImageUrl, WikiClient};
use wiremock::matchers::{method, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn fetches_image_url() -> Result<(), Box<dyn std::error::Error>> {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(query_param("action", "query"))
        .and(query_param("format", "json"))
        .and(query_param("formatversion", "2"))
        .and(query_param("prop", "imageinfo"))
        .and(query_param("iiprop", "url"))
        .and(query_param("titles", "File:Img221107.png"))
        .respond_with(ResponseTemplate::new(200).set_body_json(PAGE_1.clone()))
        .mount(&mock_server)
        .await;

    let client = WikiClient::new(mock_server.uri(), "N/A");
    let response = client.image_url("File:Img221107.png").await?;

    assert_eq!(response, Some("https://puyonexus.com/mediawiki/images/0/03/Img221107.png".to_string()));

    Ok(())
}

// #[tokio::test]
// async fn fetches_image_url_real() -> Result<(), Box<dyn std::error::Error>> {
//     let client = WikiClient::new("https://puyonexus.com/mediawiki/api.php", "N/A");

//     let result = client
//         .image_url("File:Img221107.png")
//         .await?;

//     assert_eq!(result, Some("https://puyonexus.com/mediawiki/images/0/03/Img221107.png".to_string()));
    
//     Ok(())
// }

lazy_static! {
    static ref PAGE_1: Value = json!({
        "continue": {
          "iistart": "2020-02-21T02:18:05Z",
          "continue": "||"
        },
        "query": {
          "pages": [
            {
              "pageid": 30171,
              "ns": 6,
              "title": "File:Img221107.png",
              "imagerepository": "local",
              "imageinfo": [
                {
                  "url": "https://puyonexus.com/mediawiki/images/0/03/Img221107.png",
                  "descriptionurl": "https://puyonexus.com/wiki/File:Img221107.png",
                  "descriptionshorturl": "https://puyonexus.com/mediawiki/index.php?curid=30171"
                }
              ]
            }
          ]
        }
      });
}
