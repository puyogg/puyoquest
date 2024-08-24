use lazy_static::lazy_static;
use serde_json::{json, Value};
use wiki::wiki_client::{PageImageFilenames, WikiClient};
use wiremock::{
    matchers::{method, path, query_param, query_param_contains, query_param_is_missing},
    Mock, MockServer, ResponseTemplate,
};

const PAGE_NAME: &str = "PPQ:Debug_Master_Sig/★7";

#[tokio::test]
async fn fetches_all_images_on_page() -> Result<(), Box<dyn std::error::Error>> {
    let mock_server = MockServer::start().await;

    mock_others(
        &mock_server,
        &get_continue(PAGE_1.clone()).await,
        PAGE_2.clone(),
    )
    .await;

    mock_others(
        &mock_server,
        &get_continue(PAGE_2.clone()).await,
        PAGE_3.clone(),
    )
    .await;

    mock_others(
        &mock_server,
        &get_continue(PAGE_3.clone()).await,
        PAGE_4.clone(),
    )
    .await;

    mock_others(
        &mock_server,
        &get_continue(PAGE_4.clone()).await,
        PAGE_5.clone(),
    )
    .await;

    mock_others(
        &mock_server,
        &get_continue(PAGE_5.clone()).await,
        PAGE_6.clone(),
    )
    .await;

    mock_first(&mock_server).await;

    let client = WikiClient::new(mock_server.uri(), "N/A");
    let result = client.page_image_filenames(PAGE_NAME).await?;
    // println!("{:?}", result);

    assert_eq!(result.len(), 60);

    Ok(())
}

// #[tokio::test]
// async fn fetches_all_images_on_page_real() -> Result<(), Box<dyn std::error::Error>> {
//     let client = WikiClient::new("https://puyonexus.com/mediawiki/api.php", "N/A");

//     let result = client
//         .page_image_filenames("PPQ:Debug_Master_Sig/★7")
//         .await?;
//     println!("{:?}", result);

//     // Jun 27, 2024
//     assert_eq!(result.len(), 60);

//     Ok(())
// }

async fn mock_first(mock_server: &MockServer) -> () {
    Mock::given(method("GET"))
        .and(path(""))
        .and(query_param("action", "query"))
        .and(query_param("format", "json"))
        .and(query_param("prop", "images"))
        .and(query_param("titles", PAGE_NAME))
        .and(query_param("formatversion", "2"))
        .and(query_param_is_missing("continue"))
        .and(query_param_is_missing("imcontinue"))
        .respond_with(ResponseTemplate::new(200).set_body_json(PAGE_1.clone()))
        .mount(&mock_server)
        .await;
}

async fn mock_others(mock_server: &MockServer, imcontinue: &str, page: Value) -> () {   
    Mock::given(method("GET"))
        .and(path(""))
        .and(query_param("action", "query"))
        .and(query_param("format", "json"))
        .and(query_param("prop", "images"))
        .and(query_param("titles", PAGE_NAME))
        .and(query_param("formatversion", "2"))
        .and(query_param_contains("imcontinue", imcontinue))
        .and(query_param_contains("continue", "||"))
        .respond_with(ResponseTemplate::new(200).set_body_json(page))
        .mount(&mock_server)
        .await;
}

async fn get_continue(value: Value) -> String {
    let v = value
        .get("continue")
        .unwrap()
        .get("imcontinue")
        .unwrap()
        .as_str().unwrap();

    String::from(v)
}

lazy_static! {
    static ref PAGE_1: serde_json::Value = json!({
      "continue": {
        "imcontinue": "47716|Img221107.png",
        "continue": "||"
      },
      "query": {
        "normalized": [
          {
            "fromencoded": false,
            "from": "PPQ:Debug_Master_Sig/★7",
            "to": "PPQ:Debug Master Sig/★7"
          }
        ],
        "pages": [
          {
            "pageid": 47716,
            "ns": 0,
            "title": "PPQ:Debug Master Sig/★7",
            "images": [
              {
                "ns": 6,
                "title": "File:Autoclns.png"
              },
              {
                "ns": 6,
                "title": "File:Dmgup.png"
              },
              {
                "ns": 6,
                "title": "File:Img124206.png"
              },
              {
                "ns": 6,
                "title": "File:Img201007.png"
              },
              {
                "ns": 6,
                "title": "File:Img203406.png"
              },
              {
                "ns": 6,
                "title": "File:Img204107.png"
              },
              {
                "ns": 6,
                "title": "File:Img204707.png"
              },
              {
                "ns": 6,
                "title": "File:Img204907.png"
              },
              {
                "ns": 6,
                "title": "File:Img207907.png"
              },
              {
                "ns": 6,
                "title": "File:Img218507.png"
              }
            ]
          }
        ]
      }
    });
    static ref PAGE_2: serde_json::Value = json!({
      "continue": {
        "imcontinue": "47716|Img233107.png",
        "continue": "||"
      },
      "query": {
        "normalized": [
          {
            "fromencoded": false,
            "from": "PPQ:Debug_Master_Sig/★7",
            "to": "PPQ:Debug Master Sig/★7"
          }
        ],
        "pages": [
          {
            "pageid": 47716,
            "ns": 0,
            "title": "PPQ:Debug Master Sig/★7",
            "images": [
              {
                "ns": 6,
                "title": "File:Img221107.png"
              },
              {
                "ns": 6,
                "title": "File:Img221306.png"
              },
              {
                "ns": 6,
                "title": "File:Img221506.png"
              },
              {
                "ns": 6,
                "title": "File:Img223006.png"
              },
              {
                "ns": 6,
                "title": "File:Img224406.png"
              },
              {
                "ns": 6,
                "title": "File:Img225707.png"
              },
              {
                "ns": 6,
                "title": "File:Img226507.png"
              },
              {
                "ns": 6,
                "title": "File:Img228206.png"
              },
              {
                "ns": 6,
                "title": "File:Img228907.png"
              },
              {
                "ns": 6,
                "title": "File:Img229907.png"
              }
            ]
          }
        ]
      }
    });
    static ref PAGE_3: serde_json::Value = json!({
      "continue": {
        "imcontinue": "47716|PPQWS2.png",
        "continue": "||"
      },
      "query": {
        "normalized": [
          {
            "fromencoded": false,
            "from": "PPQ:Debug_Master_Sig/★7",
            "to": "PPQ:Debug Master Sig/★7"
          }
        ],
        "pages": [
          {
            "pageid": 47716,
            "ns": 0,
            "title": "PPQ:Debug Master Sig/★7",
            "images": [
              {
                "ns": 6,
                "title": "File:Img233107.png"
              },
              {
                "ns": 6,
                "title": "File:Img237707.png"
              },
              {
                "ns": 6,
                "title": "File:Img241207.png"
              },
              {
                "ns": 6,
                "title": "File:Img246206.png"
              },
              {
                "ns": 6,
                "title": "File:Img246207.png"
              },
              {
                "ns": 6,
                "title": "File:Img246207 l.png"
              },
              {
                "ns": 6,
                "title": "File:Img246207 r.png"
              },
              {
                "ns": 6,
                "title": "File:Img246207 ss.png"
              },
              {
                "ns": 6,
                "title": "File:Img250116.png"
              },
              {
                "ns": 6,
                "title": "File:Img536807.png"
              }
            ]
          }
        ]
      }
    });
    static ref PAGE_4: serde_json::Value = json!({
      "continue": {
        "imcontinue": "47716|Puyo3_gummy_quest.png",
        "continue": "||"
      },
      "query": {
        "normalized": [
          {
            "fromencoded": false,
            "from": "PPQ:Debug_Master_Sig/★7",
            "to": "PPQ:Debug Master Sig/★7"
          }
        ],
        "pages": [
          {
            "pageid": 47716,
            "ns": 0,
            "title": "PPQ:Debug Master Sig/★7",
            "images": [
              {
                "ns": 6,
                "title": "File:PPQWS2.png"
              },
              {
                "ns": 6,
                "title": "File:PPQ Combin.png"
              },
              {
                "ns": 6,
                "title": "File:PPQskill2.png"
              },
              {
                "ns": 6,
                "title": "File:PlusPuyo Blue.png"
              },
              {
                "ns": 6,
                "title": "File:PlusPuyo Green.png"
              },
              {
                "ns": 6,
                "title": "File:PlusPuyo Purple.png"
              },
              {
                "ns": 6,
                "title": "File:PlusPuyo Red.png"
              },
              {
                "ns": 6,
                "title": "File:PlusPuyo Yellow.png"
              },
              {
                "ns": 6,
                "title": "File:Puyo1 gummy quest.png"
              },
              {
                "ns": 6,
                "title": "File:Puyo2 gummy quest.png"
              }
            ]
          }
        ]
      }
    });
    static ref PAGE_5: serde_json::Value = json!({
      "continue": {
        "imcontinue": "47716|Stb_shield5s.png",
        "continue": "||"
      },
      "query": {
        "normalized": [
          {
            "fromencoded": false,
            "from": "PPQ:Debug_Master_Sig/★7",
            "to": "PPQ:Debug Master Sig/★7"
          }
        ],
        "pages": [
          {
            "pageid": 47716,
            "ns": 0,
            "title": "PPQ:Debug Master Sig/★7",
            "images": [
              {
                "ns": 6,
                "title": "File:Puyo3 gummy quest.png"
              },
              {
                "ns": 6,
                "title": "File:Puyo4 gummy quest.png"
              },
              {
                "ns": 6,
                "title": "File:Puyo5 gummy quest.png"
              },
              {
                "ns": 6,
                "title": "File:Stb atk.png"
              },
              {
                "ns": 6,
                "title": "File:Stb chest.png"
              },
              {
                "ns": 6,
                "title": "File:Stb crown.png"
              },
              {
                "ns": 6,
                "title": "File:Stb hp.png"
              },
              {
                "ns": 6,
                "title": "File:Stb rcv.png"
              },
              {
                "ns": 6,
                "title": "File:Stb shield1.png"
              },
              {
                "ns": 6,
                "title": "File:Stb shield2s.png"
              }
            ]
          }
        ]
      }
    });
    static ref PAGE_6: serde_json::Value = json!({
      "batchcomplete": true,
      "query": {
        "normalized": [
          {
            "fromencoded": false,
            "from": "PPQ:Debug_Master_Sig/★7",
            "to": "PPQ:Debug Master Sig/★7"
          }
        ],
        "pages": [
          {
            "pageid": 47716,
            "ns": 0,
            "title": "PPQ:Debug Master Sig/★7",
            "images": [
              {
                "ns": 6,
                "title": "File:Stb shield5s.png"
              },
              {
                "ns": 6,
                "title": "File:Veget 1001.png"
              },
              {
                "ns": 6,
                "title": "File:Veget 1002.png"
              },
              {
                "ns": 6,
                "title": "File:Veget 1003.png"
              },
              {
                "ns": 6,
                "title": "File:Veget 3001.png"
              },
              {
                "ns": 6,
                "title": "File:Veget 3002.png"
              },
              {
                "ns": 6,
                "title": "File:Veget 3003.png"
              },
              {
                "ns": 6,
                "title": "File:Veget 5001.png"
              },
              {
                "ns": 6,
                "title": "File:Wildcard 52 2.png"
              },
              {
                "ns": 6,
                "title": "File:Wildcard 60 2.png"
              }
            ]
          }
        ]
      }
    });
}
