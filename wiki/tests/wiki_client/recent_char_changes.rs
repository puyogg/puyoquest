use std::collections::HashSet;

use chrono::{SecondsFormat, TimeZone, Utc};
use lazy_static::lazy_static;
use serde_json::{json, Value};
use wiki::wiki_client::{RecentCharChanges, WikiClient};
use wiremock::{
    matchers::{method, query_param, query_param_is_missing},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn fetches_recent_changes_real() -> Result<(), Box<dyn std::error::Error>> {
    let client = WikiClient::new("https://puyonexus.com/mediawiki/api.php", "N/A");

    let start = Utc.with_ymd_and_hms(2024, 6, 24, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 6, 27, 0, 0, 0).unwrap();
    let response = client.recent_char_changes(start, end).await?;

    let expected_char_ids: HashSet<String> =
        vec!["2012"].iter().map(|s| String::from(*s)).collect();

    assert_eq!(response, expected_char_ids);

    Ok(())
}

#[tokio::test]
async fn fetches_recent_changes() -> Result<(), Box<dyn std::error::Error>> {
    let mock_server = MockServer::start().await;
    let start = Utc.with_ymd_and_hms(2024, 6, 24, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 6, 27, 0, 0, 0).unwrap();
    let rcstart = start.to_rfc3339_opts(SecondsFormat::Millis, true);
    let rcend = end.to_rfc3339_opts(SecondsFormat::Millis, true);

    Mock::given(method("GET"))
        .and(query_param("action", "query"))
        .and(query_param("format", "json"))
        .and(query_param("list", "recentchanges"))
        .and(query_param("formatversion", "2"))
        .and(query_param("rcstart", &rcstart))
        .and(query_param("rcend", &rcend))
        .and(query_param("rcdir", "newer"))
        .and(query_param("rcnamespace", "10"))
        .and(query_param("rcprop", "title"))
        .and(query_param("rclimit", "500"))
        .and(query_param_is_missing("continue"))
        .and(query_param_is_missing("rccontinue"))
        .respond_with(ResponseTemplate::new(200).set_body_json(PAGE_1.clone()))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(query_param("action", "query"))
        .and(query_param("format", "json"))
        .and(query_param("list", "recentchanges"))
        .and(query_param("formatversion", "2"))
        .and(query_param("rcstart", &rcstart))
        .and(query_param("rcend", &rcend))
        .and(query_param("rcdir", "newer"))
        .and(query_param("rcnamespace", "10"))
        .and(query_param("rcprop", "title"))
        .and(query_param("rclimit", "500"))
        .and(query_param("continue", "-||"))
        .and(query_param("rccontinue", get_continue(PAGE_1.clone())))
        .respond_with(ResponseTemplate::new(200).set_body_json(PAGE_2.clone()))
        .mount(&mock_server)
        .await;

    let client = WikiClient::new(mock_server.uri(), "N/A");
    let response = client.recent_char_changes(start, end).await?;

    let expected_char_ids: HashSet<String> = vec![
        "4477", "4177", "2449", "3265", "3457", "4457", "4035", "2477", "1477", "3477", "5477",
        "5465", "2478",
    ]
    .iter()
    .map(|s| String::from(*s))
    .collect();

    assert_eq!(response, expected_char_ids);

    Ok(())
}

fn get_continue(value: Value) -> String {
    let v = value
        .get("continue")
        .unwrap()
        .get("rccontinue")
        .unwrap()
        .as_str()
        .unwrap();

    String::from(v)
}

lazy_static! {
    static ref PAGE_1: Value = json!({
      "batchcomplete": true,
      "continue": {
        "rccontinue": "20240625181859|235469",
        "continue": "-||"
      },
      "query": {
        "recentchanges": [
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:PPQ skilltext/nextamp"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:PPQ eventicon"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:147707"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:147707"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:147707"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:247706"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:147706"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:347706"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:147707"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:247707"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:347707"
          },
          {
            "type": "new",
            "ns": 10,
            "title": "Template:2478"
          },
          {
            "type": "new",
            "ns": 10,
            "title": "Template:247806"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:147706"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:147707"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:247706"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:247707"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:347706"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:347707"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:247806"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:1477"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:2477"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:3477"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:147717"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:247717"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:347717"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:247706"
          },
          {
            "type": "new",
            "ns": 10,
            "title": "Template:4477"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:4477"
          },
          {
            "type": "new",
            "ns": 10,
            "title": "Template:5477"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:S477"
          },
          {
            "type": "new",
            "ns": 10,
            "title": "Template:447706"
          }
        ]
      }
    });
    static ref PAGE_2: Value = json!({
      "batchcomplete": true,
      "query": {
        "recentchanges": [
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:347707"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:247706"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:4177"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:546506"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:244906"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:326506"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:345706"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:445706"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:Acq info/WSV"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:Acq info/PPEX"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:4035"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:Acq info/MS"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:247707"
          },
          {
            "type": "edit",
            "ns": 10,
            "title": "Template:CAgtd ticket"
          }
        ]
      }
    });
}
