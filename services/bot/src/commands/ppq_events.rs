use crate::{aws::s3, util::PLACEHOLDER_CARD_ICON};

use super::{Context, Error};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use poise::serenity_prelude::{CreateAttachment, CreateEmbed};
use ppq_imageproc::card_rows;
use sdk::apis::events_api;
use url::Url;

const ATTACHMENT_NAME: &'static str = "news.png";

/// Fetch this month's PPQ Events (JST) and the time remaining
#[poise::command(slash_command)]
pub async fn ppqevents(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();

    ctx.defer().await?;

    let mut schedule = events_api::events_get(&data.api_config).await?;

    let mut ongoing_events: Vec<String> = Vec::new();
    for event in &schedule.ongoing {
        let ending_soon = event.remaining_seconds.map(|s| s < 86400).unwrap_or(false);
        let ends_in_hours = event.remaining_seconds.map(|s| s / (60 * 60));
        let name = match (&event.name, &event.jp_name) {
            (Some(name), Some(jp_name)) => format!("{} ({})", name, jp_name),
            (Some(name), None) => name.to_string(),
            (None, Some(jp_name)) => format!("?? ({})", jp_name),
            _ => "???".to_string(),
        };

        let text = match &event.link {
            Some(l) => format!("[{}]({})", name, l.replace(" ", "_")),
            None => name,
        };
        let text = match ending_soon {
            true => match ends_in_hours {
                Some(hours) => format!("- (⏳{}) {}", hours, text),
                None => format!("- (⏳) {}", text),
            },
            false => format!("- {}", text),
        };
        ongoing_events.push(text);
    }
    let ongoing_events = ongoing_events.join("\n");

    let mut upcoming_events: Vec<String> = Vec::new();
    for event in &schedule.upcoming {
        let starting_soon = event.remaining_seconds.map(|s| s < 86400).unwrap_or(false);
        let name = match (&event.name, &event.jp_name) {
            (Some(name), Some(jp_name)) => format!("{} ({})", name, jp_name),
            (Some(name), None) => name.to_string(),
            (None, Some(jp_name)) => format!("?? ({})", jp_name),
            _ => "???".to_string(),
        };

        let text = match &event.link {
            Some(l) => format!("[{}]({})", name, l.replace(" ", "_")),
            None => name,
        };
        let text = match starting_soon {
            true => format!("- ⏰ {}", text),
            false => format!("- {}", text),
        };
        upcoming_events.push(text);
    }
    let upcoming_events = upcoming_events.join("\n");

    let mut other_events: Vec<String> = Vec::new();
    for event in &schedule.other {
        let soon = event.remaining_seconds.map(|s| s < 86400).unwrap_or(false);
        let name = match (&event.name, &event.jp_name) {
            (Some(name), Some(jp_name)) => format!("{} ({})", name, jp_name),
            (Some(name), None) => name.to_string(),
            (None, Some(jp_name)) => format!("?? ({})", jp_name),
            _ => "???".to_string(),
        };

        let text = match &event.link {
            Some(l) => format!("[{}]({})", name, l.replace(" ", "_")),
            None => name,
        };
        let text = match soon {
            true => format!("- ⏰ {}", text),
            false => format!("- {}", text),
        };
        other_events.push(text);
    }
    let other_events = other_events.join("\n");

    let mut all_events = schedule.ongoing;
    all_events.append(&mut schedule.upcoming);

    let icon_futures = all_events
        .into_iter()
        .map(|event| {
            let s3_key = event
                .icon_url
                .as_ref()
                .map(|url| {
                    let key = Url::parse(&url).ok().map(|u| u.path()[1..].to_string());
                    key
                })
                .flatten();
            s3_key
        })
        .map(|key| fetch_image_or_use_placeholder(&data.aws_client.s3, key.clone()));
    let icons = futures::stream::iter(icon_futures)
        .buffered(5)
        .collect::<Vec<_>>()
        .await;
    let image_refs = icons.iter().map(|i| i.as_slice()).collect::<Vec<&[u8]>>();
    let thumbnail = card_rows(192, 192, 6, image_refs).ok();

    let attachment = thumbnail.map(|t| CreateAttachment::bytes(t, ATTACHMENT_NAME));

    let embed = CreateEmbed::default().image(format!("attachment://{ATTACHMENT_NAME}"));

    let embed = if ongoing_events.len() > 0 {
        embed.field("Ongoing Events", ongoing_events, false)
    } else {
        embed
    };
    let embed = if upcoming_events.len() > 0 {
        embed.field("Upcoming Events", upcoming_events, false)
    } else {
        embed
    };
    let embed = if other_events.len() > 0 {
        embed.field("Other Events", other_events, false)
    } else {
        embed
    };

    let reply = poise::CreateReply::default();

    let reply = match attachment {
        None => reply.embed(embed),
        Some(a) => reply.attachment(a).embed(embed),
    };

    ctx.send(reply).await?;

    Ok(())
}

async fn fetch_image_or_use_placeholder(
    s3_client: &aws_sdk_s3::Client,
    key: Option<String>,
) -> Vec<u8> {
    let key = match key {
        Some(k) => k,
        None => return PLACEHOLDER_CARD_ICON.to_vec(),
    };

    let image = s3::get_object(s3_client, "api-pn-image-cache", key)
        .await
        .map(|i| i.as_ref().to_owned())
        .unwrap_or(PLACEHOLDER_CARD_ICON.to_vec());

    image
}

fn ends_in_hours(seconds: i64) {
    let hours = seconds / (60 * 60);
}
