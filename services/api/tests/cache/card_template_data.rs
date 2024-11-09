use poem_openapi::types::ToJSON;
use serde_json::{json, Value};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::common::seed::seed_arle;
use crate::common::{create_test_client, create_test_pool, seed, IntTestResult};

const WIKI_ARLE_7: &str = r#"{{Card info/{{{1|icon}}}
|code=201207|rarity=7
|name=Arle
|ase={{PPQ skilltext dummy}}
}}"#;

#[tokio::test]
async fn fetches_from_wiki_creates_cache() -> IntTestResult {
    let mock_pn_base = MockServer::start().await;
    let mock_pn_api = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/Template:{}", &seed::cards::ARLE_07.card_id)))
        .respond_with(ResponseTemplate::new(200).set_body_string(WIKI_ARLE_7))
        .mount(&mock_pn_base)
        .await;

    Mock::given(method("GET"))
        .and(path(""))
        .and(query_param("action", "parse"))
        .and(query_param("text", "{{PPQ skilltext dummy}}"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "parse": {
                "text": "RESOLVED_SKILL_TEXT",
                "images": [],
                "categories": [],
            }
        })))
        .mount(&mock_pn_api)
        .await;

    let (client, test_db_name, redis_client, ..) =
        create_test_client(&mock_pn_api.uri(), &mock_pn_base.uri()).await?;
    let pool = create_test_pool(&test_db_name).await?;

    seed_arle(&pool).await?;

    let expected_json = seed::cards::ARLE_07.to_json().unwrap();

    Ok(())
}
