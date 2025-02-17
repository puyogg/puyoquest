use std::{thread, time::Duration};

use indexers::env::EnvConfig;
use tokio_retry::{
    strategy::{jitter, ExponentialBackoff},
    RetryIf,
};
use wiki::wiki_client::{AllCharIds, WikiClient};
use wiki_indexers::card_indexer::{error::CardIndexerError, indexer::CardIndexer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let env = EnvConfig::new().await.unwrap();

    let request_interval: u64 = std::env::var("REQUEST_INTERVAL")
        .unwrap_or("1000".to_string())
        .parse::<u64>()
        .unwrap();

    let wiki_client = WikiClient::new("https://puyonexus.com/mediawiki/api.php", "N/A");
    let char_ids = wiki_client
        .all_char_ids()
        .await
        .expect("Fetch all char_ids");

    let indexer = CardIndexer::new(&env.ppq_api_base_url);

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    for char_id in char_ids {
        let _result = RetryIf::spawn(
            retry_strategy.clone(),
            || indexer.index_char(&char_id),
            |e: &CardIndexerError| is_retryable_error(&e),
        )
        .await;

        thread::sleep(Duration::from_millis(request_interval));
    }

    Ok(())
}

fn is_retryable_error(e: &CardIndexerError) -> bool {
    match e {
        CardIndexerError::FetchTemplateError(_fetch_template_error) => true,
        CardIndexerError::FetchCardIdsError(_card_and_material_ids_error) => true,
        CardIndexerError::UpdateCharacterError(_error) => true,
        CardIndexerError::UpdateCardError(_error) => true,
        CardIndexerError::UpdateAliasError(_error) => true,

        CardIndexerError::InvalidCharOrCardId(_) => false,
        CardIndexerError::SerdeJsonError(_error) => false,
        CardIndexerError::CardMissingKeyValue(_) => false,
        CardIndexerError::RarityModifierParsingError(_parsing_error) => false,
    }
}
