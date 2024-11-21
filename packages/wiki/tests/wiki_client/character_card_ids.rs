use wiki::wiki_client::{CardAndMaterialIds, CharacterCardIds, WikiClient};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const CHAR_TEMPLATE: &'static str = r#"{{Char info/{{{1|line}}}|size={{{size}}}
|code=3018|mat=3505
|name=Lemres|main=Lemres
|jpname=レムレス
|color=Green
|type1=Balance
|type2=Single
|voicetrans=V3018

|card1=301802
|card2=301803
|card3=301804
|card4=301805
|card5=301806
|card6=301807
|mat1=350504
|mat2=350505
|mat3=350506

|etym=
{{PPQ etym|301803|as|parfait|french|parfait}}
{{PPQ etym|301804|as|monter|french|to rise}}
{{PPQ etym|301805|as|confiture|french|jam}}
{{PPQ etym|301806|as|glaçage|french|icing}}
{{PPQ etym|301807|as|forêt noire|french|Black Forest (cake)}}

|fint=2013/05/01
|acqe=[[PPQ:Candy Festival|Candy Festival]] / [[PPQ:Lemres Returns|Lemres Returns]] / [[PPQ:Lemres Intrudes|Lemres Intrudes]]
|acqx=[[PPQ:PuyoPoint Exchange#3018|PuyoPoint Exchange]]
|acqg=Magic Stone Gacha (limited)

|stb=STBmajorbalasls
|stbc=350106|stbcn=5
|stbm1=350105|stbm1n=3
|stbm2=350105|stbm2n=3
|stbm3=350105|stbm3n=3
}}"#;

#[tokio::test]
#[ignore]
async fn fetches_card_and_material_ids_real() -> Result<(), Box<dyn std::error::Error>> {
    let client = WikiClient::new("N/A", "https://puyonexus.com/wiki");

    let result = client.character_card_ids("3018").await.unwrap();

    let expected_result = CardAndMaterialIds {
        card_ids: vec!["301802", "301803", "301804", "301805", "301806", "301807"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        material_ids: vec!["350504", "350505", "350506"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };

    assert_eq!(result, expected_result);

    Ok(())
}

#[tokio::test]
async fn fetches_card_and_material_ids() -> Result<(), Box<dyn std::error::Error>> {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/Template:3018"))
        .respond_with(ResponseTemplate::new(200).set_body_string(CHAR_TEMPLATE))
        .mount(&mock_server)
        .await;

    let client = WikiClient::new("N/A", mock_server.uri());

    let result = client.character_card_ids("3018").await.unwrap();

    let expected_result = CardAndMaterialIds {
        card_ids: vec!["301802", "301803", "301804", "301805", "301806", "301807"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        material_ids: vec!["350504", "350505", "350506"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };

    assert_eq!(result, expected_result);

    Ok(())
}
