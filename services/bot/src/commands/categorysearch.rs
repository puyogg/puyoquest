use super::{Context, Error};

/// Find cards that match an intersection of categories
#[poise::command(slash_command)]
pub async fn categorysearch(
    ctx: Context<'_>,
    #[description = "Category name"]
    #[autocomplete = "autocomplete_category"]
    category1: String,
    #[description = "Category name"]
    #[autocomplete = "autocomplete_category"]
    category2: Option<String>,
    #[description = "Category name"]
    #[autocomplete = "autocomplete_category"]
    category3: Option<String>,
    #[description = "Category name"]
    #[autocomplete = "autocomplete_category"]
    category4: Option<String>,
    #[description = "Category name"]
    #[autocomplete = "autocomplete_category"]
    category5: Option<String>,
) -> Result<(), Error> {
    let data = ctx.data();
    
    let mut categories = vec![category1];
    let mut optional_categories: Vec<String> = [category2, category3, category4, category5]
        .into_iter()
        .flatten()
        .collect();
    categories.append(&mut optional_categories);
    let categories = categories;

    let invalid_categories = check_invalid_categories(&data.api_config, &categories).await?;
    if invalid_categories.len() > 0 {
        ctx.reply(format!("These categories are invalid: {}", invalid_categories.join(", "))).await?;
        return Ok(())
    }

    ctx.defer().await?;

    let cards = sdk::apis::cards_api::cards_category_search_get(&data.api_config, categories).await;
    let cards = match cards {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            ctx.reply("There was a problem performing your category search! Try again later.")
                .await?;
            return Ok(());
        }
    };

    ctx.reply(
        cards
            .into_iter()
            .map(|c| c.link_name)
            .collect::<Vec<String>>()
            .join(", "),
    )
    .await?;

    Ok(())
}

async fn autocomplete_category(ctx: Context<'_>, partial: &str) -> Vec<String> {
    if partial.len() == 0 {
        return vec![];
    }

    let data = ctx.data();

    let categories =
        sdk::apis::categories_api::categories_get(&data.api_config, partial, Some(5), Some(false))
            .await
            .unwrap_or(vec![]);
    categories
}

async fn check_invalid_categories(api_config: &sdk::apis::configuration::Configuration, categories: &Vec<String>) -> Result<Vec<String>, Error> {
    let mut invalid_categories: Vec<String> = Vec::new();
    for category in categories {
        let query = sdk::apis::categories_api::categories_get(api_config, &category, Some(1), Some(true)).await?;
        let maybe_match = query.get(0);
        if maybe_match.is_none() {
            invalid_categories.push(category.clone());
        }
    }
    
    Ok(invalid_categories)
}
