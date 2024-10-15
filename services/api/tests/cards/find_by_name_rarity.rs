use api::aliases::types::AliasCreate;
use api::cards::template_data::CardTemplateData;
use api::{cards::types::CardCreate, characters::types::CharacterCreate};
use poem::http::StatusCode;
use poem_openapi::types::ToJSON;
use redis::AsyncCommands;
use serde_json::{json, Value};
use sqlx::PgPool;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::common::{create_test_client, create_test_pool, seed};

async fn test_seed(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let character = CharacterCreate::from(seed::characters::ARLE.clone());
    let card = CardCreate::from(seed::cards::ARLE_07.clone());
    let alias = AliasCreate::from(seed::aliases::ARLE_ALIAS_ORIGINAL.clone());

    api::characters::upsert::upsert(&pool, &seed::characters::ARLE.char_id, &character)
        .await
        .unwrap();
    api::cards::upsert::upsert(&pool, &card).await.unwrap();
    api::aliases::upsert(&pool, &seed::aliases::ARLE_ALIAS_ORIGINAL.alias, &alias)
        .await
        .unwrap();

    Ok(())
}

const WIKI_ARLE_7: &str = r#"{{Card info/{{{1|icon}}}
|code=201207|rarity=7
|name=Arle
|ase={{PPQ skilltext dummy}}
}}"#;

#[tokio::test]
async fn fetches_template_from_wiki_creates_cache() -> Result<(), Box<dyn std::error::Error>> {
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

    let (client, test_db_name, redis_client) =
        create_test_client(&mock_pn_api.uri(), &mock_pn_base.uri()).await?;
    let pool = create_test_pool(&test_db_name).await?;

    test_seed(&pool).await?;

    let expected_json = seed::cards::ARLE_07.to_json().unwrap();

    // Redis cache should be empty before making response
    let mut redis_conn = redis_client.conn.clone();
    let cached_json = redis_conn
        .get::<String, Option<String>>(
            redis_client.prefixed(format!("template:{}", &seed::cards::ARLE_07.card_id).as_str()),
        )
        .await?;
    assert!(cached_json.is_none());

    let response = client
        .get("/cards")
        .query("name", &"Arle")
        .query("rarity", &"7")
        .send()
        .await;
    response.assert_status(StatusCode::OK);
    response.assert_json(&expected_json).await;

    let expected_wiki_template = serde_json::from_value::<CardTemplateData>(
        expected_json.get("wiki_template").unwrap().clone(),
    )
    .unwrap();
    let cached_wiki_template = redis_conn
        .get::<String, Option<String>>(
            redis_client.prefixed(format!("template:{}", &seed::cards::ARLE_07.card_id).as_str()),
        )
        .await?
        .map(|j: String| serde_json::from_str::<Value>(&j).unwrap())
        .unwrap();
    let cached_wiki_template =
        serde_json::from_value::<CardTemplateData>(cached_wiki_template).unwrap();
    assert_eq!(expected_wiki_template, cached_wiki_template);

    Ok(())
}

#[tokio::test]
async fn fetches_template_from_cache() -> Result<(), Box<dyn std::error::Error>> {
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

    let (client, test_db_name, redis_client) =
        create_test_client("N/A", &mock_server.uri()).await?;
    println!("{test_db_name}");
    let pool = create_test_pool(&test_db_name).await?;

    test_seed(&pool).await?;

    let expected_json = seed::cards::ARLE_07.to_json().unwrap();
    let cached_wiki_template = expected_json.get("wiki_template").unwrap().clone();

    let mut redis_conn = redis_client.conn.clone();
    let _: std::result::Result<String, redis::RedisError> = redis_conn
        .set(
            redis_client.prefixed(format!("template:{}", &seed::cards::ARLE_07.card_id).as_str()),
            cached_wiki_template.to_json_string(),
        )
        .await;

    let response = client
        .get("/cards")
        .query("name", &"Arle")
        .query("rarity", &"7")
        .send()
        .await;
    response.assert_status(StatusCode::OK);
    response.assert_json(&expected_json).await;

    Ok(())
}
