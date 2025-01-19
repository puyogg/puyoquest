use std::sync::LazyLock;

use crate::aws::s3;
use bytes::Bytes;
use fancy_regex::Regex;
use poise::serenity_prelude::{CreateAttachment, CreateEmbed};
use poise::CreateReply;
use ppq_imageproc::IconSide;
use sdk::apis::{cards_api, characters_api};
use sdk::models::Card;

use crate::util;
use crate::util::parse_card_query::{parse_alias_and_rarity, AliasAndRarityQuery};
use crate::util::sort_rarity::sort_rarity;

use super::{Context, Data, Error};

const PLACEHOLDER_ICON: &'static [u8] = include_bytes!("../images/Img000000.png");

enum LookupError {
    AliasNotFound(String),
    RarityLookupFailure(String),
}

#[poise::command(slash_command)]
pub async fn iq(
    ctx: Context<'_>,
    #[description = "Format: [card:side:variant] Your quote here. -- Ex: [Arle 7:right:ep] Hi Carbuncle!"]
    #[min_length = 1]
    #[max_length = 160]
    quote1: String,
    #[description = "Format: [card:side:variant] Your quote here. -- Ex: [Arle 7:right:ep] Hi Carbuncle!"]
    #[min_length = 1]
    #[max_length = 160]
    quote2: Option<String>,
    #[description = "Format: [card:side:variant] Your quote here. -- Ex: [Arle 7:right:ep] Hi Carbuncle!"]
    #[min_length = 1]
    #[max_length = 160]
    quote3: Option<String>,
    #[description = "Format: [card:side:variant] Your quote here. -- Ex: [Arle 7:right:ep] Hi Carbuncle!"]
    #[min_length = 1]
    #[max_length = 160]
    quote4: Option<String>,
    #[description = "Format: [card:side:variant] Your quote here. -- Ex: [Arle 7:right:ep] Hi Carbuncle!"]
    #[min_length = 1]
    #[max_length = 160]
    quote5: Option<String>,
    #[description = "Format: [card:side:variant] Your quote here. -- Ex: [Arle 7:right:ep] Hi Carbuncle!"]
    #[min_length = 1]
    #[max_length = 160]
    quote6: Option<String>,
    #[description = "Format: [card:side:variant] Your quote here. -- Ex: [Arle 7:right:ep] Hi Carbuncle!"]
    #[min_length = 1]
    #[max_length = 160]
    quote7: Option<String>,
    #[description = "Format: [card:side:variant] Your quote here. -- Ex: [Arle 7:right:ep] Hi Carbuncle!"]
    #[min_length = 1]
    #[max_length = 160]
    quote8: Option<String>,
) -> Result<(), Error> {
    let data = ctx.data();

    let input_queries: Vec<Option<String>> = vec![
        Some(quote1),
        quote2,
        quote3,
        quote4,
        quote5,
        quote6,
        quote7,
        quote8,
    ];

    // Parse card query, icon side, icon type, and quote
    let mut invalid_parsed_queries: Vec<String> = Vec::new();
    let mut parsed_queries: Vec<(AliasAndRarityQuery, IconSide, IconType, String)> = Vec::new();
    for (i, query) in input_queries.iter().enumerate() {
        let query = match query {
            Some(q) => q,
            None => continue,
        };

        let parse_result = parse_slash_command_quote(&query);
        match parse_result {
            Ok(q) => parsed_queries.push(q),
            Err(e) => match e {
                QuoteParsingError::InvalidIconQuery => invalid_parsed_queries.push(format!(
                    "Invalid icon query in slash command parameter: quote{}",
                    i + 1
                )),
                QuoteParsingError::InvalidOrEmptyQuote => invalid_parsed_queries.push(format!(
                    "Invalid quote in slash command parameter: quote{}",
                    i + 1
                )),
            },
        };
    }

    if invalid_parsed_queries.len() > 0 {
        let error_list = invalid_parsed_queries.join("\n");
        let error_message = format!("Error! Your request had these errors:\n\n{error_list}");

        ctx.send(
            CreateReply::default()
                .reply(true)
                .ephemeral(true)
                .content(error_message),
        )
        .await?;
        return Ok(());
    }

    ctx.defer().await?;

    // Perform alias and rarity queries
    let quote_futures =
        parsed_queries
            .into_iter()
            .map(|(card_query, icon_side, icon_type, quote)| {
                resolve_icon(data, card_query, icon_side, icon_type, quote)
            });
    let quotes = futures::future::join_all(quote_futures).await;

    let mut quote_errors: Vec<String> = Vec::new();
    let mut iq_input: Vec<(Bytes, IconSide, String)> = Vec::new();
    let mut alt_text_quotes: Vec<String> = Vec::new();
    for quote in quotes {
        match quote {
            Ok((bytes, icon_side, icon_type, quote, card)) => {
                let (name, rarity, variant) = match &card {
                    Some(c) => {
                        let name = c.name.clone();
                        let rarity = c.rarity_modifier.clone().unwrap_or(c.rarity.clone());
                        let variant = match icon_type {
                            IconType::Normal => None,
                            IconType::DualShift => Some("DS"),
                            IconType::ExtraPower => Some("EP"),
                            IconType::ExtraPowerDualShift => Some("EPDS"),
                        };
                        (name, rarity, variant)
                    }
                    None => (String::from("UNKNOWN CHARACTER"), String::from("??"), None),
                };
                let alt_text_quote = match variant {
                    Some(v) => format!("{name} ★{rarity} ({v}): {quote}"),
                    None => format!("{name} ★{rarity}: {quote}"),
                };
                alt_text_quotes.push(alt_text_quote);
                iq_input.push((bytes, icon_side, quote));
            }
            Err(e) => match e {
                LookupError::AliasNotFound(a) => {
                    quote_errors.push(format!("Failed to lookup card from query: {a}"));
                }
                LookupError::RarityLookupFailure(r) => {
                    quote_errors.push(format!("Failed to find character: {r}"));
                }
            },
        }
    }
    let iq_input = iq_input
        .iter()
        .map(|(bytes, icon_side, quote)| (bytes.as_ref(), *icon_side, quote.as_str()))
        .collect::<Vec<(&[u8], IconSide, &str)>>();

    if quote_errors.len() > 0 {
        let error_list = quote_errors.join("\n");
        let error_message = format!("Error! Your request had these errors:\n\n{error_list}");

        ctx.send(
            CreateReply::default()
                .reply(true)
                .ephemeral(true)
                .content(error_message),
        )
        .await?;
        return Ok(());
    }

    let incorrect_quote = ppq_imageproc::incorrect_quote(iq_input);
    match incorrect_quote {
        Ok(img) => {
            let alt_text = format_alt_text(alt_text_quotes);
            let attachment = CreateAttachment::bytes(img, "iq.png").description(alt_text);
            let reply = poise::CreateReply::default().attachment(attachment);
            ctx.send(reply).await?;
        }
        Err(_) => {
            ctx.send(CreateReply::default().reply(true).ephemeral(true).content("There was an issue creating the image for your iq request. Try again later or tell S2L.")).await?;
        }
    }

    Ok(())
}

enum IconType {
    Normal,
    DualShift,
    ExtraPower,
    ExtraPowerDualShift,
}

enum QuoteParsingError {
    InvalidIconQuery,
    InvalidOrEmptyQuote,
}

pub static RE_IQ_CARD_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\[(.*?)\](.*)").unwrap());
fn parse_slash_command_quote(
    quote: &str,
) -> Result<(AliasAndRarityQuery, IconSide, IconType, String), QuoteParsingError> {
    let quote = quote.trim();
    let captures = RE_IQ_CARD_TAG
        .captures(quote)
        .map_err(|_| QuoteParsingError::InvalidIconQuery)?
        .ok_or(QuoteParsingError::InvalidIconQuery)?;

    let card_side_variant = captures.get(1).ok_or(QuoteParsingError::InvalidIconQuery)?;
    let splits = card_side_variant.as_str().split(":").collect::<Vec<&str>>();
    let card_query = splits
        .get(0)
        .map(|c| parse_alias_and_rarity(c))
        .ok_or(QuoteParsingError::InvalidIconQuery)?;
    let side = splits
        .get(1)
        .map(|s| {
            let s = s.trim().to_lowercase();
            match s.as_ref() {
                "left" => IconSide::LEFT,
                "l" => IconSide::LEFT,
                "right" => IconSide::RIGHT,
                "r" => IconSide::RIGHT,
                _ => IconSide::LEFT,
            }
        })
        .unwrap_or(IconSide::LEFT);
    let variant = splits
        .get(2)
        .map(|v| {
            let v = v.trim().replace(" ", "").to_lowercase();
            match v.as_ref() {
                "normal" => IconType::Normal,
                "n" => IconType::Normal,
                "dualshift" => IconType::DualShift,
                "ds" => IconType::DualShift,
                "extrapower" => IconType::ExtraPower,
                "ep" => IconType::ExtraPower,
                "extrapowerdualshift" => IconType::ExtraPowerDualShift,
                "epds" => IconType::ExtraPowerDualShift,
                "dualshiftextrapower" => IconType::ExtraPowerDualShift,
                "dsep" => IconType::ExtraPowerDualShift,
                _ => IconType::Normal,
            }
        })
        .unwrap_or(IconType::Normal);

    let query_alone = captures
        .get(2)
        .map(|q| q.as_str().trim())
        .ok_or(QuoteParsingError::InvalidOrEmptyQuote)?
        .to_string();
    if query_alone.len() == 0 {
        return Err(QuoteParsingError::InvalidOrEmptyQuote);
    }

    Ok((card_query, side, variant, query_alone))
}

async fn resolve_icon(
    data: &Data,
    card_query: AliasAndRarityQuery,
    icon_side: IconSide,
    icon_type: IconType,
    quote: String,
) -> Result<(Bytes, IconSide, IconType, String, Option<Card>), LookupError> {
    let s3_client = &data.aws_client.s3;

    if let Some(q) = card_query.query {
        let card = cards_api::cards_get(&data.api_config, Some(&q.alias), Some(&q.rarity)).await;

        if let Ok(c) = card {
            // Get the card Icon
            let pn_url = match icon_type {
                IconType::Normal => &c.icons.normal,
                IconType::DualShift => &c.icons.dual_shift,
                IconType::ExtraPower => &c.icons.extra_power,
                IconType::ExtraPowerDualShift => &c.icons.extra_power_dual_shift,
            };

            let pn_url = match pn_url {
                Some(p) => &Some(p.to_string()),
                None => &c.icons.normal,
            };

            let icon = match pn_url {
                Some(url) => {
                    let key = util::pn_url_to_s3_key(&url);
                    let icon = match key {
                        Some(k) => s3::get_object(&s3_client, &s3::IMAGE_CACHE_BUCKET_NAME, &k)
                            .await
                            .unwrap_or(Bytes::from_static(PLACEHOLDER_ICON)),
                        None => Bytes::from_static(PLACEHOLDER_ICON),
                    };
                    icon
                }
                None => Bytes::from_static(PLACEHOLDER_ICON),
            };

            return Ok((icon, icon_side, icon_type, quote, Some(c)));
        }
    }

    // Fall back to querying the character, and then querying for the highest rarity
    let character =
        characters_api::characters_get(&data.api_config, Some(&card_query.fallback)).await;
    let character = match character {
        Ok(c) => {
            let c = c.get(0);
            match c {
                Some(c) => c.clone(),
                None => return Err(LookupError::AliasNotFound(card_query.fallback.to_string())),
            }
        }
        Err(_) => return Err(LookupError::AliasNotFound(card_query.fallback.to_string())),
    };
    let cards_and_materials = sdk::apis::characters_api::characters_id_cards_get(
        &data.api_config,
        &character.char_id,
        Some("false"),
    )
    .await;
    let cards_and_materials = match cards_and_materials {
        Ok(c) => c,
        Err(_) => {
            return Err(LookupError::RarityLookupFailure(
                card_query.fallback.to_string(),
            ))
        }
    };
    let mut cards = cards_and_materials.cards;
    cards.sort_by(|a, b| {
        sort_rarity(
            (&a.rarity, &a.rarity_modifier),
            (&b.rarity, &b.rarity_modifier),
        )
    });
    let rarest_card = cards.last();

    let icon = match rarest_card {
        Some(c) => {
            let icon_option = match icon_type {
                IconType::Normal => &c.icons.normal,
                IconType::DualShift => &c.icons.dual_shift,
                IconType::ExtraPower => &c.icons.extra_power,
                IconType::ExtraPowerDualShift => &c.icons.extra_power_dual_shift,
            };

            match icon_option {
                Some(url) => {
                    let key = util::pn_url_to_s3_key(&url);
                    let icon = match key {
                        Some(k) => s3::get_object(&s3_client, &s3::IMAGE_CACHE_BUCKET_NAME, &k)
                            .await
                            .unwrap_or(Bytes::from_static(PLACEHOLDER_ICON)),
                        None => Bytes::from_static(PLACEHOLDER_ICON),
                    };
                    icon
                }
                None => Bytes::from_static(PLACEHOLDER_ICON),
            }
        }
        None => Bytes::from_static(PLACEHOLDER_ICON),
    };

    let rarest_card = rarest_card.cloned();

    return Ok((icon, icon_side, icon_type, quote, rarest_card));
}

fn format_alt_text(quote_lines: Vec<String>) -> String {
    let joined = quote_lines.join("\n\n");

    let should_truncate = joined.len() > 1000;

    match should_truncate {
        true => format!("{}... (truncated)", &joined[..990]),
        false => joined,
    }
}
