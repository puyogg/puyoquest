use lazy_static::lazy_static;
use serde_json::json;
use wiki::wiki_client::{CardCategories, WikiClient};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn fetches_all_categories_from_paginated_api() -> Result<(), Box<dyn std::error::Error>> {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(""))
        .and(query_param("action", "query"))
        .and(query_param("format", "json"))
        .and(query_param("prop", "categories"))
        .and(query_param("titles", "PPQ:Ally & Rafisol/★7"))
        .and(query_param("formatversion", "2"))
        .and(query_param("continue", "||"))
        .and(query_param("clcontinue", "49133|PPQ:HP_Recovery_AS"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ALLY_AND_RAFISOL7_CATS_PAGE_2.clone()),
        )
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path(""))
        .and(query_param("action", "query"))
        .and(query_param("format", "json"))
        .and(query_param("prop", "categories"))
        .and(query_param("titles", "PPQ:Ally & Rafisol/★7"))
        .and(query_param("formatversion", "2"))
        .and(query_param("continue", "||"))
        .and(query_param(
            "clcontinue",
            "49133|PPQ:Special_Skill_Source_(color)_LS",
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ALLY_AND_RAFISOL7_CATS_PAGE_3.clone()),
        )
        .mount(&mock_server)
        .await;

    // Have to specify the first request last.
    // Since it has identical but less params than the other pages, wiremock could potentially match it first.
    Mock::given(method("GET"))
        .and(path(""))
        .and(query_param("action", "query"))
        .and(query_param("format", "json"))
        .and(query_param("prop", "categories"))
        .and(query_param("titles", "PPQ:Ally & Rafisol/★7"))
        .and(query_param("formatversion", "2"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ALLY_AND_RAFISOL7_CATS_PAGE_1.clone()),
        )
        .mount(&mock_server)
        .await;

    let client = WikiClient::new(mock_server.uri(), "N/A");

    let result = client.card_categories("PPQ:Ally & Rafisol/★7").await?;

    assert_eq!(result.len(), 24);

    Ok(())
}

// Uncomment this if you want to see a real request to the Puyo Nexus wiki
// #[tokio::test]
// async fn fetches_all_ally_and_rafisol_7star_categories() -> Result<(), Box<dyn std::error::Error>> {
//     let client = WikiClient::new("https://puyonexus.com/mediawiki/api.php", "N/A");

//     let result = client.card_categories("PPQ:Ally & Rafisol/★7").await?;
//     println!("{:?}", result);

//     // Jun 24, 2024
//     assert_eq!(result.len(), 24);

//     Ok(())
// }

lazy_static! {
    static ref ALLY_AND_RAFISOL7_CATS_PAGE_1: serde_json::Value = json!({
      "continue": {
        "clcontinue": "49133|PPQ:HP_Recovery_AS",
        "continue": "||"
      },
      "query": {
        "normalized": [
          {
            "fromencoded": false,
            "from": "PPQ:Ally_&_Rafisol/★7",
            "to": "PPQ:Ally & Rafisol/★7"
          }
        ],
        "pages": [
          {
            "pageid": 49133,
            "ns": 0,
            "title": "PPQ:Ally & Rafisol/★7",
            "categories": [
              {
                "ns": 14,
                "title": "Category:PPQ:Boss!? Combination"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Card Cost 68"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Cards first introduced in February 2024"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Cards inflicting Color Damage Taken Up"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Cards inflicting Heart-pounding Luwa Mode"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Cards with Extra Training Board enabled"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Chaining Coefficient Up LS"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Color Damage Taken Up AS"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Girls Combination"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Green Color"
              }
            ]
          }
        ]
      }
    });
    static ref ALLY_AND_RAFISOL7_CATS_PAGE_2: serde_json::Value = json!({
      "continue": {
        "clcontinue": "49133|PPQ:Special_Skill_Source_(color)_LS",
        "continue": "||"
      },
      "query": {
        "normalized": [
          {
            "fromencoded": false,
            "from": "PPQ:Ally_&_Rafisol/★7",
            "to": "PPQ:Ally & Rafisol/★7"
          }
        ],
        "pages": [
          {
            "pageid": 49133,
            "ns": 0,
            "title": "PPQ:Ally & Rafisol/★7",
            "categories": [
              {
                "ns": 14,
                "title": "Category:PPQ:HP Recovery AS"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:HP Up ESS"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Heart-pounding Luwa Mode AS"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Heroines Combination"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Many Mysteries Combination"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Mode Activation AS"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Purple Color"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Puyo Fest Keyword"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Recover Type"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Single Attack"
              }
            ]
          }
        ]
      }
    });
    static ref ALLY_AND_RAFISOL7_CATS_PAGE_3: serde_json::Value = json!({
      "batchcomplete": true,
      "query": {
        "normalized": [
          {
            "fromencoded": false,
            "from": "PPQ:Ally_&_Rafisol/★7",
            "to": "PPQ:Ally & Rafisol/★7"
          }
        ],
        "pages": [
          {
            "pageid": 49133,
            "ns": 0,
            "title": "PPQ:Ally & Rafisol/★7",
            "categories": [
              {
                "ns": 14,
                "title": "Category:PPQ:Special Skill Source (color) LS"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Special Skill Source LS"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:Trace Capacity Up ESS"
              },
              {
                "ns": 14,
                "title": "Category:PPQ:★7 Cards"
              }
            ]
          }
        ]
      }
    });
}
