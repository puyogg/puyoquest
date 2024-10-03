use serde_json::{json, Value};
use wiki::wiki_client::{FetchTemplate, WikiClient};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ARLE: &str = r#"{{Char info/{{{1|line}}}|size={{{size}}}|main=Arle Nadja
|code=2012
|name=Arle
|jpname=アルル
|color=Blue
|type1=Attack
|type2=Single
|voicetrans=V2012

|card1=201203
|card2=201204
|card3=201205
|card4=201206
|card5=201207

|acqg=Magic Stone Gacha / Silver Ticket Gacha / Gold Ticket Gacha / Premium Ticket Gacha / [[PPQ:Once-per-day Free Gacha|Once-per-day Free Gacha]]

|manzai=3
|tokkun=Tmainatkas7ls7
}}"#;

#[tokio::test]
async fn fetches_template() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/Template:2012"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ARLE))
        .mount(&mock_server)
        .await;
    let client = WikiClient::new("N/A", mock_server.uri());
    let result = client.fetch_template("2012").await.unwrap();

    let Value::Object(expected_value) = json!({
        "acqg": "Magic Stone Gacha / Silver Ticket Gacha / Gold Ticket Gacha / Premium Ticket Gacha / [[PPQ:Once-per-day Free Gacha|Once-per-day Free Gacha]]",
        "card1": "201203",
        "card2": "201204",
        "card3": "201205",
        "card4": "201206",
        "card5": "201207",
        "code": "2012",
        "color": "Blue",
        "jpname": "アルル",
        "main": "Arle Nadja",
        "manzai": "3",
        "name": "Arle",
        "size": "{{{size}}}",
        "tokkun": "Tmainatkas7ls7",
        "type1": "Attack",
        "type2": "Single",
        "voicetrans": "V2012"
    }) else {
        unreachable!();
    };

    assert_eq!(result, Value::from(expected_value));
}
