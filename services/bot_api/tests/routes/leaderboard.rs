use bot_api::routes::leaderboard::UserRanking;

use crate::common::{IntTestResult, create_test_client};

#[tokio::test]
async fn new_user() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let response = client
        .post("/leaderboards/serverId/gameType/userId_1/increment")
        .send()
        .await;

    response.assert_status_is_ok();
    response
        .assert_json(&UserRanking {
            user_id: "userId_1".to_string(),
            server_id: "serverId".to_string(),
            game_type: "gameType".to_string(),
            correct: 1,
            ranking: 1,
        })
        .await;

    Ok(())
}

#[tokio::test]
async fn increments() -> IntTestResult<()> {
    let client = create_test_client().await?;

    for _i in 0..4 {
        client
            .post("/leaderboards/serverId/gameType/userId_1/increment")
            .send()
            .await;
    }

    let response = client
        .post("/leaderboards/serverId/gameType/userId_1/increment")
        .send()
        .await;

    response.assert_status_is_ok();
    response
        .assert_json(&UserRanking {
            user_id: "userId_1".to_string(),
            server_id: "serverId".to_string(),
            game_type: "gameType".to_string(),
            correct: 5,
            ranking: 1,
        })
        .await;

    Ok(())
}

#[tokio::test]
async fn top_10_less_than_10_players() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let set_score = async |user_id: &str, score: i32| {
        for _i in 0..score {
            client
                .post(format!(
                    "/leaderboards/serverId/gameType/{user_id}/increment"
                ))
                .send()
                .await;
        }
    };

    set_score("A", 15).await;
    set_score("C", 12).await;
    set_score("B", 15).await;
    set_score("D", 11).await;
    set_score("E", 10).await;

    let response = client
        .get("/leaderboards/serverId/gameType/top")
        .send()
        .await;

    response.assert_status_is_ok();

    let top = response.json().await;
    let top = top.value().deserialize::<Vec<UserRanking>>();
    let ids = top.into_iter().map(|r| r.user_id).collect::<Vec<String>>();
    assert_eq!(ids, vec!["A", "B", "C", "D", "E"]);

    Ok(())
}

#[tokio::test]
async fn top_10_more_than_10_players() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let set_score = async |user_id: &str, score: i32| {
        for _i in 0..score {
            client
                .post(format!(
                    "/leaderboards/serverId/gameType/{user_id}/increment"
                ))
                .send()
                .await;
        }
    };

    set_score("B", 15).await;
    set_score("A", 15).await;
    set_score("C", 12).await;
    set_score("D", 11).await;
    set_score("E", 10).await;
    set_score("F", 9).await;
    set_score("H", 8).await;
    set_score("G", 9).await;
    set_score("I", 7).await;
    set_score("J", 6).await;
    set_score("K", 5).await;
    set_score("L", 4).await;
    set_score("M", 2).await;

    let response = client
        .get("/leaderboards/serverId/gameType/top")
        .send()
        .await;

    response.assert_status_is_ok();

    let top = response.json().await;
    let top = top.value().deserialize::<Vec<UserRanking>>();
    let ids = top.into_iter().map(|r| r.user_id).collect::<Vec<String>>();
    assert_eq!(ids, vec!["B", "A", "C", "D", "E", "F", "G", "H", "I", "J"]);

    Ok(())
}
