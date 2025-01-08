use super::{Context, Error};
use crate::aws::s3;
use bytes::Bytes;
use futures::StreamExt;
use poise::serenity_prelude::{CreateAttachment, CreateEmbed};
use ppq_imageproc::card_rows;
use sdk::models::Card;
use url::Url;
use urlencoding::encode;

const ATTACHMENT_NAME: &'static str = "categorysearch.png";

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
    let s3_client = &data.aws_client.s3;

    let mut categories = vec![category1];
    let mut optional_categories: Vec<String> = [category2, category3, category4, category5]
        .into_iter()
        .flatten()
        .collect();
    categories.append(&mut optional_categories);
    let categories = categories;

    let invalid_categories = check_invalid_categories(&data.api_config, &categories).await?;
    if invalid_categories.len() > 0 {
        ctx.reply(format!(
            "These categories are invalid: {}",
            invalid_categories.join(", ")
        ))
        .await?;
        return Ok(());
    }

    ctx.defer().await?;

    let cards =
        sdk::apis::cards_api::cards_category_search_get(&data.api_config, categories.clone()).await;
    let cards = match cards {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            ctx.reply("There was a problem performing your category search! Try again later.")
                .await?;
            return Ok(());
        }
    };

    if cards.len() == 0 {
        ctx.reply("Your search matched 0 cards.").await?;
        return Ok(());
    }

    let urls = cards
        .iter()
        .filter_map(|c| {
            let url = c
                .icons
                .normal
                .as_ref()
                .map(|u| Url::parse(&u).ok())
                .flatten();

            match url {
                None => return None,
                Some(url) => {
                    // remove leading /
                    let key = url.path()[1..].to_string();
                    Some(key)
                }
            }
        })
        .collect::<Vec<String>>();

    let s3_futures = urls
        .into_iter()
        .map(|key| s3::get_object(&s3_client, "api-pn-image-cache", key));

    let images = futures::stream::iter(s3_futures)
        .buffered(30)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .filter_map(|f| f.ok())
        .collect::<Vec<Bytes>>();
    let image_refs = images.iter().map(|i| i.as_ref()).collect::<Vec<&[u8]>>();

    let grid = card_rows(192, 192, 5, image_refs);
    let attachment = grid
        .ok()
        .map(|g| CreateAttachment::bytes(g, ATTACHMENT_NAME));

    let title_embed = result_count_embed(cards.len(), &categories);
    let link_embeds = card_links(&cards);

    let mut reply = poise::CreateReply::default().embed(title_embed);
    match link_embeds {
        CardLinkEmbed::Ok(vec) => {
            for em in vec {
                reply = reply.embed(em);
            }
        }
        CardLinkEmbed::TooLarge => (),
    };
    let reply = match attachment {
        None => reply,
        Some(a) => {
            let attachment_embed = CreateEmbed::default().attachment(ATTACHMENT_NAME);
            reply.attachment(a).embed(attachment_embed)
        }
    };

    ctx.send(reply).await?;

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

async fn check_invalid_categories(
    api_config: &sdk::apis::configuration::Configuration,
    categories: &Vec<String>,
) -> Result<Vec<String>, Error> {
    let mut invalid_categories: Vec<String> = Vec::new();
    for category in categories {
        let query =
            sdk::apis::categories_api::categories_get(api_config, &category, Some(1), Some(true))
                .await?;
        let maybe_match = query.get(0);
        if maybe_match.is_none() {
            invalid_categories.push(category.clone());
        }
    }

    Ok(invalid_categories)
}

fn result_count_embed(count: usize, categories: &Vec<String>) -> CreateEmbed {
    // TODO: need to use RESOLVED category names
    let category_links = categories
        .iter()
        .map(|category| {
            let space_to_underline = category.replace(" ", "_");
            let uri_category = encode(&space_to_underline);
            format!(
                "[Category:PPQ:{}](https://puyonexus.com/wiki/Category:PPQ:{})",
                category, uri_category
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    CreateEmbed::default()
        .title(format!("Category Search Result: {} cards", count))
        .description(category_links)
}

enum CardLinkEmbed {
    Ok(Vec<CreateEmbed>),
    TooLarge,
}

fn card_links(cards: &Vec<Card>) -> CardLinkEmbed {
    let markdown_links = cards
        .iter()
        .map(|c| {
            let name = c.link_name.replace("/★7", "");
            format!("[[{}]]({})", name, c.url)
        })
        .collect::<Vec<String>>();

    let joined_total = markdown_links.join(" ");
    if joined_total.len() > 5500 {
        return CardLinkEmbed::TooLarge;
    }

    let mut embeds: Vec<CreateEmbed> = Vec::new();
    let mut current_embed_content = String::from("");
    for link in markdown_links {
        let length_to_add = 1 + link.len();
        if length_to_add + current_embed_content.len() > 4000 {
            let embed = CreateEmbed::default().description(current_embed_content.trim());
            embeds.push(embed);
            current_embed_content = String::from("");
        }

        current_embed_content.push_str(&format!(" {}", link));
    }
    if current_embed_content.len() > 0 {
        let embed = CreateEmbed::default().description(current_embed_content.trim());
        embeds.push(embed);
    }

    CardLinkEmbed::Ok(embeds)
}
