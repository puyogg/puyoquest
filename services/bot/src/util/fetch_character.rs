use anyhow::anyhow;
use futures::TryFutureExt;
use sdk::{
    apis::configuration::Configuration,
    models::{Alias, CardsAndMaterials, Character},
};

pub async fn fetch_character(
    api_config: &Configuration,
    char_id: &str,
) -> Result<(Character, CardsAndMaterials), anyhow::Error> {
    futures::try_join!(
        sdk::apis::characters_api::characters_id_get(api_config, &char_id)
            .map_err(|_| anyhow!("Failed to fetch character.")),
        sdk::apis::characters_api::characters_id_cards_get(api_config, &char_id, Some("false"),)
            .map_err(|_| anyhow!("Failed to fetch cards and materials.")),
    )
}

pub async fn fetch_character_and_aliases(
    api_config: &Configuration,
    char_id: &str,
) -> Result<(Character, CardsAndMaterials, Vec<Alias>), anyhow::Error> {
    futures::try_join!(
        sdk::apis::characters_api::characters_id_get(api_config, &char_id)
            .map_err(|_| anyhow!("Failed to fetch character.")),
        sdk::apis::characters_api::characters_id_cards_get(api_config, &char_id, Some("false"),)
            .map_err(|_| anyhow!("Failed to fetch cards and materials.")),
        sdk::apis::aliases_api::aliases_get(api_config, Some(&char_id), None, None)
            .map_err(|_| anyhow!("Failed to fetch aliases.")),
    )
}
