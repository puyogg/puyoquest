use chrono::Utc;
use indexers::card_indexer::indexer::Indexer;
use indexers::db;
use indexers::discord::WebhookLogger;
use indexers::env::EnvConfig;
use std::thread;
use std::time::Duration;
use wiki::wiki_client::RecentCharChanges;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let env = EnvConfig::new().await.unwrap();
    let pool = db::create_pool(&env.environment, &env.db_connection_string)
        .await
        .unwrap();
    db::init_db(&pool).await.unwrap();

    let logger = WebhookLogger::new(&env.webhook_url);
    let wiki_client = wiki::wiki_client::WikiClient::new(
        "https://puyonexus.com/mediawiki/api.php",
        "https://puyonexus.com/wiki",
    );

    loop {
        let last_run = db::get_last_run(&pool).await.unwrap();
        let now = Utc::now();

        let recent_changes = wiki_client.recent_char_changes(last_run, now).await;
        match recent_changes {
            Err(e) => {
                let message = format!("Error fetching recent change: {:?}", e);
                let message = &message[..std::cmp::min(message.len(), 1800)];
                logger.log(message).await.unwrap();
            }
            Ok(recent_changes) => {
                println!(
                    "Found {} char_ids updated between {} and {} to index!",
                    recent_changes.len(),
                    last_run.to_rfc3339(),
                    now.to_rfc3339()
                );
                for (i, char_id) in recent_changes.iter().enumerate() {
                    print!("{}. ", i + 1);
                    let indexer = Indexer::new(&env.ppq_api_base_url, None, &char_id);

                    for i in 0..5 {
                        let update = indexer.update_character_and_cards().await;
                        match update {
                            Ok(_) => break,
                            Err(e) => {
                                println!("Error indexing char_id {}: {:?}", char_id, e);

                                if i == 4 {
                                    logger
                                        .log(&format!("Error indexing char_id {}", char_id))
                                        .await
                                        .unwrap();
                                }

                                thread::sleep(Duration::from_secs(5));
                                continue;
                            }
                        }
                    }
                    thread::sleep(Duration::from_secs(2));
                }

                db::update_last_run(&pool, &now).await.unwrap();
            }
        };

        println!("Finished.");
        thread::sleep(Duration::from_secs(1800));
    }
}
