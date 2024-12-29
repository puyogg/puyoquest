use api::cards::get_by_id::query_get_by_id;
use api::db::create_pool;
use api::env_config;
use fancy_regex::Regex;
use sqlx::Row;
use utils::normalize_name;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = &*env_config::ENV;

    let pool = create_pool(&env.environment, &env.db_connection_string).await?;

    let results = sqlx::query(
        r#"
        SELECT card_id
        FROM card
    "#,
    )
    .fetch_all(&pool)
    .await?;

    let card_ids: Vec<String> = results
        .into_iter()
        .map(|r| r.get::<String, usize>(0 as usize))
        .collect();

    let rarity_suffix: Regex = Regex::new(r"\/★.*").unwrap();

    for (i, card_id) in card_ids.iter().enumerate() {
        let card_db = query_get_by_id(&pool, &card_id).await?;
        let card_db = match card_db {
            Some(c) => c,
            None => continue,
        };

        let already_has_rarity = rarity_suffix.is_match(&card_db.link_name);
        let already_has_rarity = match already_has_rarity {
            Ok(b) => b,
            Err(e) => {
                println!("Regex error: {}", e);
                continue;
            }
        };
        if already_has_rarity {
            continue;
        }

        let rarity_link_name = format!("{}/★{}", &card_db.name, &card_db.rarity);
        let rarity_link_name_normalized = normalize_name(&rarity_link_name);

        let update_count = sqlx::query(
            r#"
            UPDATE card
            SET
                link_name = $2,
                link_name_normalized = $3
            WHERE card_id = $1
        "#,
        )
        .bind(&card_id)
        .bind(&rarity_link_name)
        .bind(&rarity_link_name_normalized)
        .execute(&pool)
        .await?
        .rows_affected();

        if update_count == 1 {
            println!("{}. {} → {}", i, &card_db.link_name, &rarity_link_name);
        }

        if i >=3 {
            break;
        }
    }
    Ok(())
}
