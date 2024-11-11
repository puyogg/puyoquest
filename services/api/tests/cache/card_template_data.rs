use api::cards::template_data::CardTemplateData;
use poem_openapi::types::ToJSON;
use redis::AsyncCommands;
use serde_json::{json, Value};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::common::seed::seed_arle;
use crate::common::{create_test_client, create_test_pool, seed, IntTestResult};
use api::cache::card_template_data;

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

    let (_, test_db_name, redis_client, wiki_client, ..) =
        create_test_client(&mock_pn_api.uri(), &mock_pn_base.uri()).await?;
    let pool = create_test_pool(&test_db_name).await?;

    seed_arle(&pool).await?;

    // Redis cache should be empty before making response
    let mut redis_conn = redis_client.conn.clone();
    let cached_json = redis_conn
        .get::<String, Option<String>>(
            redis_client.prefixed(format!("template:{}", &seed::cards::ARLE_07.card_id).as_str()),
        )
        .await?;
    assert!(cached_json.is_none());

    // Fetches from wiki because cache is empty
    let card_template = card_template_data(&redis_client, &wiki_client, "201207").await?;
    let expected_template = seed::cards::ARLE_07.clone().wiki_template;
    assert_eq!(card_template, expected_template);

    // Cache contains card template from wiki now
    let cached_wiki_template = redis_conn
        .get::<String, Option<String>>(
            redis_client.prefixed(format!("template:{}", &seed::cards::ARLE_07.card_id).as_str()),
        )
        .await?
        .map(|j: String| serde_json::from_str::<Value>(&j).unwrap())
        .map(|v| serde_json::from_value::<CardTemplateData>(v).unwrap())
        .unwrap();
    assert_eq!(cached_wiki_template, expected_template);

    Ok(())
}

#[tokio::test]
async fn fetches_template_from_cache() -> IntTestResult {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/Template:{}", &seed::cards::ARLE_07.card_id)))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{{Card info/{{{1|icon}}}
|code=201207|rarity=7
|name=SHOULD NOT HAVE BEEN FETCHED
}}"#,
        ))
        .mount(&mock_server)
        .await;

    let (_, test_db_name, redis_client, wiki_client, ..) = create_test_client("N/A", &mock_server.uri()).await?;
    let pool = create_test_pool(&test_db_name).await?;
    seed_arle(&pool).await?;

    let cached_template = seed::cards::ARLE_07.clone().wiki_template;

    let mut redis_conn = redis_client.conn.clone();
    let _: std::result::Result<String, redis::RedisError> = redis_conn
        .set(
            redis_client.prefixed(format!("template:{}", &seed::cards::ARLE_07.card_id).as_str()),
            cached_template.to_json_string(),
        )
        .await;

    let result = card_template_data(&redis_client, &wiki_client, "201207").await?;

    assert_ne!(result.name, "SHOULD NOT HAVE BEEN FETCHED");
    assert_eq!(result, cached_template);

    Ok(())
}
