use api::cards::lore::{Lore, MonologueLine, WikiLore};
use poem::http::StatusCode;
use redis::AsyncCommands;

use crate::common::seed::seed_arle;
use crate::common::{create_test_client, create_test_pool};

#[tokio::test]
async fn fetches_lore() -> Result<(), Box<dyn std::error::Error>> {
    let (client, test_db_name, redis_client, _, ..) = create_test_client("N/A", "N/A").await?;
    let mut redis_conn = redis_client.conn.clone();
    let key = redis_client.prefixed("lore:201207");
    let pool = create_test_pool(&test_db_name).await?;
    seed_arle(&pool).await?;

    let expected_wiki_lore = WikiLore {
        code: "201207".to_string(),
        ft: Some("異世界からぷよと一緒に飛ばされてきたとっても元気な魔導師のタマゴ。冒険が大好きで、アヤシイ洞窟や遺跡などを見かけると、つい入りたくなってしまう。".to_string()),
        fta: Some("A spirited, developing magician from another world who flew in alongside Puyo. She loves a good adventure, and whenever she sees a mysterious looking cave or ruins, she feels compelled to enter.".to_string()),
        ftc: Some("Translator\n[Kirub][1]\nEditor\nNone\n\n[1]: /wiki/User:Kirub".to_string()),
        ft1: Some("ねえねえ、この前の冒険のつづき…今から行ってみない？".to_string()),
        fta1: Some("Hey, hey, after that last adventure... how about we have another go?".to_string()),
        ft2: Some("あっちの方で、ぷよ勝負がはじまったみたい！次はボクもまぜてもらおーっと！".to_string()),
        fta2: Some("It looks like there's a Puyo battle going on over there! I'll have to join in on the next one!".to_string()),
        ft3: Some("この島って、小さいようでとっても大きいよねぜんぶ見て回るのに、どれくらいかかるのかなあ？".to_string()),
        fta3: Some("This island seems tiny, but it's really huge, so I wonder how long it'll take to explore it all?".to_string()),
    };
    let cache_string = serde_json::to_string(&expected_wiki_lore).unwrap();
    let _ = redis_conn
        .set::<&str, String, Option<String>>(&key, cache_string.clone())
        .await?;

    let response = client.get("/cards/201207/lore").send().await;
    response.assert_status(StatusCode::OK);

    response
        .assert_json(Lore {
            card_id: "201207".to_string(),
            flavor_text_jp: expected_wiki_lore.ft.clone(),
            flavor_text_en: expected_wiki_lore.fta.clone(),
            monologue_lines: vec![
                MonologueLine {
                    jp: expected_wiki_lore.ft1,
                    en: expected_wiki_lore.fta1,
                },
                MonologueLine {
                    jp: expected_wiki_lore.ft2,
                    en: expected_wiki_lore.fta2,
                },
                MonologueLine {
                    jp: expected_wiki_lore.ft3,
                    en: expected_wiki_lore.fta3,
                },
            ],
            translator: Some("Kirub".to_string()),
            editor: None,
        })
        .await;

    Ok(())
}
