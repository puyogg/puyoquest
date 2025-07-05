use bot_api::routes::{leaderboard_channel::{LeaderboardChannel, LeaderboardChannelCreate}, server_settings::ServerSettings};
use poem::http::StatusCode;

use crate::common::{create_test_client, IntTestResult};

#[tokio::test]
async fn set_leaderboard_channel() -> IntTestResult<()> {
    let client = create_test_client().await?;

    client
        .post("/server-settings")
        .body_json(&ServerSettings {
            server_id: "serverId".to_string(),
        })
        .send()
        .await;

    let response = client
        .post("/leaderboard-channel/serverId/gameType")
        .body_json(&LeaderboardChannelCreate {
            channel_id: "channelId".into(),
        })
        .send()
        .await;

    response.assert_status_is_ok();
    response.assert_json(LeaderboardChannel {
        server_id: "serverId".into(),
        game_type: "gameType".into(),
        channel_id: "channelId".into(),
    }).await;

    Ok(())
}

#[tokio::test]
async fn get_leaderboard_channel() -> IntTestResult<()> {
    let client = create_test_client().await?;

    client
        .post("/server-settings")
        .body_json(&ServerSettings {
            server_id: "serverId".to_string(),
        })
        .send()
        .await;

    let upsert_response = client
        .post("/leaderboard-channel/serverId/gameType")
        .body_json(&LeaderboardChannelCreate {
            channel_id: "channelId".into(),
        })
        .send()
        .await;
    upsert_response.assert_status_is_ok();

    let response = client
        .get("/leaderboard-channel/serverId/gameType")
        .send()
        .await;

    response.assert_status_is_ok();
    response.assert_json(LeaderboardChannel {
        server_id: "serverId".into(),
        game_type: "gameType".into(),
        channel_id: "channelId".into(),
    }).await;

    Ok(())
}

#[tokio::test]
async fn upserts_leaderboard_channel() -> IntTestResult<()> {
    let client = create_test_client().await?;

    client
        .post("/server-settings")
        .body_json(&ServerSettings {
            server_id: "serverId".to_string(),
        })
        .send()
        .await;

    let response = client
        .post("/leaderboard-channel/serverId/gameType")
        .body_json(&LeaderboardChannelCreate {
            channel_id: "channelId".into(),
        })
        .send()
        .await;
    response.assert_status_is_ok();
    response.assert_json(LeaderboardChannel {
        server_id: "serverId".into(),
        game_type: "gameType".into(),
        channel_id: "channelId".into(),
    }).await;

    let response = client
        .post("/leaderboard-channel/serverId/gameType")
        .body_json(&LeaderboardChannelCreate {
            channel_id: "channelId2".into(),
        })
        .send()
        .await;
    response.assert_status_is_ok();
    response.assert_json(LeaderboardChannel {
        server_id: "serverId".into(),
        game_type: "gameType".into(),
        channel_id: "channelId2".into(),
    }).await;

    let response = client
        .get("/leaderboard-channel/serverId/gameType")
        .send()
        .await;
    response.assert_status_is_ok();
    response.assert_json(LeaderboardChannel {
        server_id: "serverId".into(),
        game_type: "gameType".into(),
        channel_id: "channelId2".into(),
    }).await;

    Ok(())
}

#[tokio::test]
async fn delete_leaderboard_channel() -> IntTestResult<()> {
    let client = create_test_client().await?;

    client
        .post("/server-settings")
        .body_json(&ServerSettings {
            server_id: "serverId".to_string(),
        })
        .send()
        .await;

    let upsert_response = client
        .post("/leaderboard-channel/serverId/gameType")
        .body_json(&LeaderboardChannelCreate {
            channel_id: "channelId".into(),
        })
        .send()
        .await;
    upsert_response.assert_status_is_ok();

    let delete_response = client
        .delete("/leaderboard-channel/serverId/gameType")
        .send()
        .await;
    delete_response.assert_status_is_ok();

    let response = client
        .get("/leaderboard-channel/serverId/gameType")
        .send()
        .await;
    response.assert_status(StatusCode::NOT_FOUND);

    Ok(())
}
