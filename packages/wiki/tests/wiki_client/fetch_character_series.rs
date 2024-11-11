use wiki::wiki_client::{FetchCharacterSeries, WikiClient};
use wiremock::{
    matchers::{method, path, query_param, query_param_is_missing},
    Mock, MockServer, ResponseTemplate,
};
use urlencoding::encode;

// #[tokio::test]
// async fn gets_series_from_character_real() -> Result<(), Box<dyn std::error::Error>> {
//     let client = WikiClient::new(
//         "https://puyonexus.com/mediawiki/api.php",
//         "https://puyonexus.com/wiki",
//     );

//     let (series_name, is_lore) = client
//         .fetch_character_series("4203", "Legamünt")
//         .await?
//         .unwrap();

//     assert_eq!(series_name, String::from("Heavenly Knight Series"));
//     assert_eq!(is_lore, true);

//     Ok(())
// }

const LEGAMUNT_PAGE: &'static str = r#"{{4203|long}}
{{S089|short}}
{{E4203|except}}"#;

const HEAVENLY_KNIGHT_SERIES_RAW: &'static str = r#"{{Series info/{{{1|line}}}|size={{{size}}}
|code=S089|filter=t
|name=Heavenly Knight Series
|jpname=天騎士シリーズ

|char1=1089
|char2=2089
|char3=3089
|char4=4089
|char5=5089

|lore1=4203
}}"#;

#[tokio::test]
async fn gets_series_from_character() -> Result<(), Box<dyn std::error::Error>> {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/PPQ:{}", encode("Legamünt"))))
        .respond_with(ResponseTemplate::new(200).set_body_string(LEGAMUNT_PAGE))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/Template:S089"))
        .respond_with(ResponseTemplate::new(200).set_body_string(HEAVENLY_KNIGHT_SERIES_RAW))
        .mount(&mock_server)
        .await;

    let client = WikiClient::new("N/A", mock_server.uri());

    let (series_name, is_lore) = client
        .fetch_character_series("4203", "Legamünt")
        .await?
        .unwrap();

    assert_eq!(series_name, String::from("Heavenly Knight Series"));
    assert_eq!(is_lore, true);

    Ok(())
}
