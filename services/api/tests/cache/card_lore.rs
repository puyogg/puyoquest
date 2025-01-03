use crate::common::{create_test_client, IntTestResult};
use redis::AsyncCommands;

use api::{cache::card_lore_data, cards::lore::WikiLore};

#[tokio::test]
#[ignore]
async fn fetches_lore_from_wiki_real() -> IntTestResult {
    let (_, _, redis_client, wiki_client, _, ..) = create_test_client(
        "https://puyonexus.com/mediawiki/api.php",
        "https://puyonexus.com/wiki",
    )
    .await?;
    let mut redis_conn = redis_client.conn.clone();

    let key = redis_client.prefixed(&format!("lore:402207"));

    // Cache should be empty
    let current_cache: Option<String> = redis_conn.get(&key).await?;
    assert!(current_cache.is_none());

    let lore = card_lore_data(&redis_client, &wiki_client, "402207").await?;

    assert_eq!(
        lore,
        WikiLore {
            code: "402207".to_string(),
            ft: Some("さまざまな世界を旅して まわっている「時空の旅人」。 怒らせるとちょっとコワイ。".to_string()),
            fta: Some("A \"space-time traveler\" who travels around various worlds. He becomes a bit scary when agitated.".to_string()),
            ftc: Some("Translator\n[Beachedking][1]\nEditor\n[Pi][2]\n\n[1]: /wiki/User:Beachedking\n[2]: /wiki/User:Pi".to_string()),
            ft1: Some("あ、いたの？ 気付かなかった".to_string()),
            fta1: Some("Ah, you were there? I didn't notice.".to_string()),
            ft2: Some("いろんなところを旅してきたけど ここもひかくてき楽しいね".to_string()),
            fta2: Some("I've traveled to lots of different places, but coming here is also pretty fun!".to_string()),
            ft3: Some("とってもいい気分だよ くすくすくすくす…".to_string()),
            fta3: Some("I feel great! *chuckle*".to_string()),
        },
        // Lore {
        //     ft: Some("異世界からぷよと一緒に飛ばされてきたとっても元気な魔導師のタマゴ。冒険が大好きで、アヤシイ洞窟や遺跡などを見かけると、つい入りたくなってしまう。".to_string()),
        //     fta: Some("A spirited, developing magician from another world who flew in alongside Puyo. She loves a good adventure, and whenever she sees a mysterious looking cave or ruins, she feels compelled to enter.".to_string()),
        //     ftc: Some("Translator\n[Kirub][1]\nEditor\nNone\n\n[1]: /wiki/User:Kirub".to_string()),
        //     ft1: Some("ねえねえ、この前の冒険のつづき…今から行ってみない？".to_string()),
        //     fta1: Some("Hey, hey, after that last adventure... how about we have another go?".to_string()),
        //     ft2: Some("あっちの方で、ぷよ勝負がはじまったみたい！次はボクもまぜてもらおーっと！".to_string()),
        //     fta2: Some("It looks like there's a Puyo battle going on over there! I'll have to join in on the next one!".to_string()),
        //     ft3: Some("この島って、小さいようでとっても大きいよねぜんぶ見て回るのに、どれくらいかかるのかなあ？".to_string()),
        //     fta3: Some("This island seems tiny, but it's really huge, so I wonder how long it'll take to explore it all?".to_string()),
        // },
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn fetches_lore_from_wiki_multiple_translators_real() -> IntTestResult {
    let (_, _, redis_client, wiki_client, _, ..) = create_test_client(
        "https://puyonexus.com/mediawiki/api.php",
        "https://puyonexus.com/wiki",
    )
    .await?;
    let mut redis_conn = redis_client.conn.clone();

    let key = redis_client.prefixed(&format!("lore:241207"));

    // Cache should be empty
    let current_cache: Option<String> = redis_conn.get(&key).await?;
    assert!(current_cache.is_none());

    let lore = card_lore_data(&redis_client, &wiki_client, "241207").await?;

    assert_eq!(
        lore,
        WikiLore {
            code: "241207".to_string(),
            ft: Some("さまざまな世界を旅して まわっている「時空の旅人」。 怒らせるとちょっとコワイ。".to_string()),
            fta: Some("A \"space-time traveler\" who travels around various worlds. He becomes a bit scary when agitated.".to_string()),
            ftc: Some("Translator\n[Beachedking][1]\nEditor\n[Pi][2]\n\n[1]: /wiki/User:Beachedking\n[2]: /wiki/User:Pi".to_string()),
            ft1: Some("あ、いたの？ 気付かなかった".to_string()),
            fta1: Some("Ah, you were there? I didn't notice.".to_string()),
            ft2: Some("いろんなところを旅してきたけど ここもひかくてき楽しいね".to_string()),
            fta2: Some("I've traveled to lots of different places, but coming here is also pretty fun!".to_string()),
            ft3: Some("とってもいい気分だよ くすくすくすくす…".to_string()),
            fta3: Some("I feel great! *chuckle*".to_string()),
        },
    );

    Ok(())
}

#[tokio::test]
async fn fetches_lore_from_cache() -> IntTestResult {
    let (_, _, redis_client, wiki_client, _, ..) = create_test_client("N/A", "N/A").await?;

    let mut redis_conn = redis_client.conn.clone();
    let key = redis_client.prefixed(&format!("lore:402207"));

    let expected_wiki_lore = WikiLore {
        code: "402207".to_string(),
        ft: Some("さまざまな世界を旅して まわっている「時空の旅人」。 怒らせるとちょっとコワイ。".to_string()),
        fta: Some("A \"space-time traveler\" who travels around various worlds. He becomes a bit scary when agitated.".to_string()),
        ftc: Some("Translator\n[Beachedking][1]\nEditor\n[Pi][2]\n\n[1]: /wiki/User:Beachedking\n[2]: /wiki/User:Pi".to_string()),
        ft1: Some("あ、いたの？ 気付かなかった".to_string()),
        fta1: Some("Ah, you were there? I didn't notice.".to_string()),
        ft2: Some("いろんなところを旅してきたけど ここもひかくてき楽しいね".to_string()),
        fta2: Some("I've traveled to lots of different places, but coming here is also pretty fun!".to_string()),
        ft3: Some("とってもいい気分だよ くすくすくすくす…".to_string()),
        fta3: Some("I feel great! *chuckle*".to_string()),
    };
    let cache_string = serde_json::to_string(&expected_wiki_lore).unwrap();

    let _ = redis_conn
        .set::<&str, String, Option<String>>(&key, cache_string.clone())
        .await?;

    let wiki_lore = card_lore_data(&redis_client, &wiki_client, "402207").await?;

    assert_eq!(wiki_lore, expected_wiki_lore);

    Ok(())
}
