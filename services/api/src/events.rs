use std::future::IntoFuture;
use std::sync::Arc;

use crate::env_config::ENV;
use crate::{api_tag::ApiTag, aws::s3::S3BackupClient, cache::RedisClient, config::ApiConfig};
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::{Japan, Tz};
use futures::{StreamExt, TryFutureExt};
use poem::{error::InternalServerError, web::Data};
use poem_openapi::{param::Query, payload::Json, ApiResponse, Enum, Object, OpenApi};
use serde::Serialize;
use sqlx::PgPool;
use wiki::wiki_client::{FetchMonthlyEvents, FetchMonthlyEventsError, WikiClient};

use crate::cache::{card_icons, CardIconUrls};
use crate::cards;

pub struct EventsRouter;

#[derive(Debug, Enum, Clone, Serialize, PartialEq, Eq)]
#[oai(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    GuildRush,
    LimitedStory,
    Collection,
    Tournament,
    StoryQuest,
    Intrusion,
    Hunting,
    Treasure,
    Unknown,
}

impl From<wiki::wiki_client::PpqEventType> for EventType {
    fn from(value: wiki::wiki_client::PpqEventType) -> Self {
        match value {
            wiki::wiki_client::PpqEventType::GuildRush => EventType::GuildRush,
            wiki::wiki_client::PpqEventType::LimitedStory => EventType::LimitedStory,
            wiki::wiki_client::PpqEventType::Collection => EventType::Collection,
            wiki::wiki_client::PpqEventType::Tournament => EventType::Tournament,
            wiki::wiki_client::PpqEventType::StoryQuest => EventType::StoryQuest,
            wiki::wiki_client::PpqEventType::Intrusion => EventType::Intrusion,
            wiki::wiki_client::PpqEventType::Hunting => EventType::Hunting,
            wiki::wiki_client::PpqEventType::Treasure => EventType::Treasure,
            wiki::wiki_client::PpqEventType::Unknown => EventType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Object)]
pub struct PpqEvent {
    pub icon_url: Option<String>,
    pub event_type: EventType,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub remaining_seconds: Option<i64>,
    pub name: Option<String>,
    pub link: Option<String>,
    pub jp_name: Option<String>,
}

#[derive(ApiResponse)]
enum EventListResponse {
    #[oai(status = 200)]
    EventList(Json<Vec<PpqEvent>>),
}

#[OpenApi(prefix_path = "/events", tag = "ApiTag::Events")]
impl EventsRouter {
    #[oai(path = "/", method = "get")]
    async fn list(
        &self,
        api_config: Data<&Arc<ApiConfig>>,
        pool: Data<&PgPool>,
        wiki_client: Data<&wiki::wiki_client::WikiClient>,
        redis_client: Data<&Arc<RedisClient>>,
        s3_client: Data<&Arc<S3BackupClient>>,
    ) -> poem::Result<EventListResponse> {
        let pool = pool.0;
        let wiki_client = wiki_client.0;
        let redis_client = redis_client.0;
        let s3_client = s3_client.0;
        let image_base_url = api_config.0.get_image_cache_domain().await?;
        let pn_base_url = ENV.pn_wiki_base_url.clone();

        let events = wiki_client
            .monthly_events()
            .await
            .map_err(InternalServerError)?;

        let mapped_events_futures = events.into_iter().map(|event| {
            map_wiki_event(
                pool,
                redis_client,
                wiki_client,
                s3_client,
                &image_base_url,
                &pn_base_url,
                event,
            )
        });

        let stream = futures::stream::iter(mapped_events_futures).buffered(5);
        let events = stream
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .filter_map(|f| f.ok())
            .flatten()
            .collect::<Vec<PpqEvent>>();

        Ok(EventListResponse::EventList(Json(events)))
    }
}

async fn map_wiki_event(
    pool: &PgPool,
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    s3_client: &S3BackupClient,
    image_base_url: &str,
    pn_base_url: &str,
    event: wiki::wiki_client::PpqEvent,
) -> poem::Result<Option<PpqEvent>> {
    let remaining_seconds = match (&event.start, &event.end) {
        (Some(start), Some(end)) => {
            let diff = end.to_utc() - start.to_utc();
            Some(diff.num_seconds())
        }
        _ => None,
    };

    if event.icon == "000000" {
        return Ok(Some(PpqEvent {
            icon_url: None,
            event_type: EventType::from(event.r#type),
            start: event.start.map(|d| d.to_utc()),
            end: event.end.map(|d| d.to_utc()),
            remaining_seconds,
            name: event.name,
            link: event.link.map(|l| format!("{pn_base_url}/PPQ:{}", l)),
            jp_name: event.jp_name,
        }));
    }

    let card = cards::get_by_id::query_get_by_id(pool, &event.icon).await?;
    let card = match card {
        Some(c) => c,
        None => return Ok(None),
    };

    let icon_urls = card_icons(redis_client, wiki_client, s3_client, image_base_url, &card).await?;

    let mapped_event = PpqEvent {
        icon_url: icon_urls.normal,
        event_type: EventType::from(event.r#type),
        start: event.start.map(|d| d.to_utc()),
        end: event.end.map(|d| d.to_utc()),
        remaining_seconds,
        name: event.name,
        link: event.link.map(|l| format!("{}/PPQ:{}", pn_base_url, l)),
        jp_name: event.jp_name,
    };

    Ok(Some(mapped_event))
}
