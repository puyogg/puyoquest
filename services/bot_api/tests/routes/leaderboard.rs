use bot_api::routes::leaderboard::UserRanking;

use crate::common::{IntTestResult, create_test_client};
use serde_json::json;

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

#[tokio::test]
async fn leaderboard_window_near_top_or_bottom() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let increment_score = async |user_id: &str, score: i32| {
        for _i in 0..score {
            client
                .post(format!(
                    "/leaderboards/serverId/gameType/{user_id}/increment"
                ))
                .send()
                .await;
        }
    };

    increment_score("B", 15).await;
    increment_score("A", 15).await;
    increment_score("C", 12).await;
    increment_score("D", 11).await;
    increment_score("E", 10).await;
    increment_score("F", 9).await;
    increment_score("H", 8).await;
    increment_score("G", 9).await;
    increment_score("I", 7).await;
    increment_score("J", 6).await;
    increment_score("K", 5).await;
    increment_score("L", 4).await;
    increment_score("M", 2).await;

    {
        let response = client
            .get("/leaderboards/serverId/gameType/window")
            .query("user_id", &"C")
            .send()
            .await;
        response.assert_status_is_ok();
    
        let window = response.json().await;
        let window = window.value().deserialize::<Vec<UserRanking>>();
        let ids = window.into_iter().map(|r| r.user_id).collect::<Vec<String>>();
        assert_eq!(ids, vec!["B", "A", "C", "D", "E", "F", "G", "H", "I", "J"]);
    }

    {
        let response = client
            .get("/leaderboards/serverId/gameType/window")
            .query("user_id", &"L")
            .send()
            .await;
        response.assert_status_is_ok();
    
        let window = response.json().await;
        let window = window.value().deserialize::<Vec<UserRanking>>();
        let ids = window.into_iter().map(|r| r.user_id).collect::<Vec<String>>();
        assert_eq!(ids, vec!["D", "E", "F", "G", "H", "I", "J", "K", "L", "M"]);
    }

    Ok(())
}

#[tokio::test]
async fn empty_top_leaderboard() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let response = client
        .get("/leaderboards/serverId/gameType/top")
        .send()
        .await;
    response.assert_status_is_ok();

    response.assert_json(&json!([])).await;

    Ok(())
}

#[tokio::test]
async fn empty_window_leaderboard() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let response = client
        .get("/leaderboards/serverId/gameType/window")
        .query("user_id", &"C")
        .send()
        .await;
    response.assert_status_is_ok();

    response.assert_json(&json!([])).await;

    Ok(())
}

#[tokio::test]
async fn resets_all_scores() -> IntTestResult<()> {
    let client = create_test_client().await?;

    let increment_score = async |server_id: &str, user_id: &str, score: i32| {
        for _i in 0..score {
            client
                .post(format!(
                    "/leaderboards/{server_id}/gameType/{user_id}/increment"
                ))
                .send()
                .await;
        }
    };

    let fetch_leaderboard = async |server_id: &str| {
        let top_response = client
            .get(format!("/leaderboards/{server_id}/gameType/top"))
            .send()
            .await;

        top_response.assert_status_is_ok();
        let top = top_response.json().await;
        let top: Vec<UserRanking> = top.value().deserialize();
        top
    };

    increment_score("server1", "A", 5).await;
    increment_score("server2", "B", 5).await;
    increment_score("server1", "C", 3).await;

    let top_server1 = fetch_leaderboard("server1").await;
    assert_eq!(top_server1.len(), 2);
    let top_server2 = fetch_leaderboard("server2").await;
    assert_eq!(top_server2.len(), 1);

    let reset_response = client
        .delete("/leaderboards/gameType")
        .send()
        .await;
    reset_response.assert_status_is_ok();

    let top_server1 = fetch_leaderboard("server1").await;
    assert_eq!(top_server1.len(), 0);
    let top_server2 = fetch_leaderboard("server2").await;
    assert_eq!(top_server2.len(), 0);

    Ok(())
}
