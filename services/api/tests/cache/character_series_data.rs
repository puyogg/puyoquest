use redis::AsyncCommands;
use urlencoding::encode;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::common::{create_test_client, IntTestResult};
use api::cache::{character_series_data, SeriesNameLore};

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

// region: Actual Series Data
#[tokio::test]
pub async fn fetches_lore_series_data_from_wiki() -> IntTestResult {
    let mock_pn_base = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/PPQ:{}", encode("Legamünt"))))
        .respond_with(ResponseTemplate::new(200).set_body_string(LEGAMUNT_PAGE))
        .mount(&mock_pn_base)
        .await;

    Mock::given(method("GET"))
        .and(path("/Template:S089"))
        .respond_with(ResponseTemplate::new(200).set_body_string(HEAVENLY_KNIGHT_SERIES_RAW))
        .mount(&mock_pn_base)
        .await;

    let (_, _, redis_client, wiki_client, ..) =
        create_test_client("N/A", &mock_pn_base.uri()).await?;
    let mut redis_conn = redis_client.conn.clone();

    // Cache should be empty
    let cached_series_name = redis_conn
        .get::<String, Option<String>>(
            redis_client.prefixed(format!("char_series:{}", "4203").as_str()),
        )
        .await?;
    assert!(cached_series_name.is_none());

    // Returns series name from wiki
    let result = character_series_data(&redis_client, &wiki_client, "4203", "Legamünt")
        .await?
        .unwrap();
    assert_eq!(result.0, "Heavenly Knight Series".to_string());
    assert_eq!(result.1, true);

    // Cache should now contain series data
    let cached_series_data = redis_conn
        .get::<String, Option<String>>(
            redis_client.prefixed(format!("char_series:{}", "4203").as_str()),
        )
        .await?
        .map(|j| serde_json::from_str::<SeriesNameLore>(&j).unwrap())
        .unwrap();
    assert_eq!(cached_series_data.series_name, "Heavenly Knight Series");
    assert_eq!(cached_series_data.is_lore, true);

    Ok(())
}

#[tokio::test]
pub async fn fetches_series_data_from_cache() -> IntTestResult {
    let (_, _, redis_client, wiki_client, ..) = create_test_client("N/A", "N/A").await?;
    let mut redis_conn = redis_client.conn.clone();

    let cached_series_data = SeriesNameLore {
        series_name: "Heavenly Knight Series!!!".to_string(),
        is_lore: true,
    };
    let _: std::result::Result<String, redis::RedisError> = redis_conn
        .set(
            redis_client.prefixed("char_series:4203"),
            serde_json::to_string(&cached_series_data).unwrap(),
        )
        .await;

    let result = character_series_data(&redis_client, &wiki_client, "4203", "Legamünt")
        .await?
        .unwrap();
    assert_eq!(result.0, "Heavenly Knight Series!!!");
    assert_eq!(result.1, true);

    Ok(())
}
// endregion: Actual Series Data

// region: Character without series
const EVEN_DARKER_SIG_PAGE: &'static str = r#"{{5462|long}}"#;

#[tokio::test]
pub async fn fetches_char_without_series_from_wiki() -> IntTestResult {
    let mock_pn_base = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/PPQ:{}", encode("Even Darker Sig"))))
        .respond_with(ResponseTemplate::new(200).set_body_string(EVEN_DARKER_SIG_PAGE))
        .mount(&mock_pn_base)
        .await;

    let (_, _, redis_client, wiki_client, ..) =
        create_test_client("N/A", &mock_pn_base.uri()).await?;
    let mut redis_conn = redis_client.conn.clone();

    // Cache should be empty
    let cached_series_name = redis_conn
        .get::<String, Option<String>>(
            redis_client.prefixed(format!("char_series:{}", "5462").as_str()),
        )
        .await?;
    assert!(cached_series_name.is_none());

    // Returns none for series data
    let result =
        character_series_data(&redis_client, &wiki_client, "5462", "Even Darker Sig").await?;
    assert!(result.is_none());

    // Saves empty string to cache
    let cached_series_name = redis_conn
        .get::<String, Option<String>>(
            redis_client.prefixed(format!("char_series:{}", "5462").as_str()),
        )
        .await?
        .unwrap();
    assert_eq!(cached_series_name, "");

    Ok(())
}

#[tokio::test]
pub async fn fetches_char_without_series_from_cache() -> IntTestResult {
    let (_, _, redis_client, wiki_client, ..) =
        create_test_client("N/A", "N/A").await?;
    let mut redis_conn = redis_client.conn.clone();

    let _: std::result::Result<String, redis::RedisError> = redis_conn
        .set(redis_client.prefixed("char_series:5462"), "")
        .await;

    let result =
        character_series_data(&redis_client, &wiki_client, "5462", "Even Darker Sig").await?;
    assert!(result.is_none());

    Ok(())
}
// endregion
