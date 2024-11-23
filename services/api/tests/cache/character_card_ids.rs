use crate::common::{create_test_client, IntTestResult};
use api::cache;
use redis::AsyncCommands;
use wiki::wiki_client::CardAndMaterialIds;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
pub async fn fetches_card_ids_from_wiki() -> IntTestResult {
    let char_id = "3018"; // Lemres

    let mock_pn_base = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/Template:{char_id}",)))
        .respond_with(ResponseTemplate::new(200).set_body_string(LEMRES_TEMPLATE))
        .mount(&mock_pn_base)
        .await;

    let (_, _, redis_client, wiki_client, _, ..) =
        create_test_client("N/A", &mock_pn_base.uri()).await?;

    let mut redis_conn = redis_client.conn.clone();

    // Cache should be empty
    let cached_card_ids = redis_conn
        .get::<String, Option<String>>(redis_client.prefixed(&format!("cards_mats:{char_id}")))
        .await?;
    assert!(cached_card_ids.is_none());

    // Returns ids from wiki
    let result = cache::character_card_ids(&redis_client, &wiki_client, char_id, false).await?;
    assert_eq!(
        result.card_ids,
        vec!["301802", "301803", "301804", "301805", "301806", "301807"]
    );
    assert_eq!(result.material_ids, vec!["350504", "350505", "350506"]);

    // Cache should now contain ids
    let cached_cards_ids = redis_conn
        .get::<String, Option<String>>(redis_client.prefixed(&format!("cards_mats:{char_id}")))
        .await?
        .unwrap();
    let cached_cards_ids = serde_json::from_str::<CardAndMaterialIds>(&cached_cards_ids).unwrap();
    assert_eq!(
        cached_cards_ids.card_ids,
        vec!["301802", "301803", "301804", "301805", "301806", "301807"]
    );
    assert_eq!(
        cached_cards_ids.material_ids,
        vec!["350504", "350505", "350506"]
    );

    Ok(())
}

#[tokio::test]
pub async fn fetches_card_ids_from_cache() -> IntTestResult {
    let char_id = "3018";

    let (_, _, redis_client, wiki_client, _, ..) = create_test_client("N/A", "N/A").await?;

    let mut redis_conn = redis_client.conn.clone();

    let expected_card_mat_ids = CardAndMaterialIds {
        card_ids: vec!["301802", "301803", "301804", "301805", "301806", "301807"]
            .into_iter()
            .map(|v| v.to_string())
            .collect(),
        material_ids: vec!["350504", "350505", "350506"]
            .into_iter()
            .map(|v| v.to_string())
            .collect(),
    };
    let expected_string = serde_json::to_string(&expected_card_mat_ids).unwrap();
    redis_conn
        .set::<String, &str, Option<String>>(
            redis_client.prefixed(&format!("cards_mats:{char_id}")),
            &expected_string,
        )
        .await?;

    let result = cache::character_card_ids(&redis_client, &wiki_client, char_id, false).await?;

    assert_eq!(result.card_ids, expected_card_mat_ids.card_ids);
    assert_eq!(result.material_ids, expected_card_mat_ids.material_ids);

    Ok(())
}

const LEMRES_TEMPLATE: &'static str = r#"{{Char info/{{{1|line}}}|size={{{size}}}
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
