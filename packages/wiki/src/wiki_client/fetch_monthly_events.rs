use std::sync::LazyLock;

use crate::util::parse_template;
use async_trait::async_trait;
use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Utc};
use chrono_tz::{Japan, Tz};
use fancy_regex::Regex;

#[derive(Debug)]
pub enum FetchMonthlyEventsError {
    ChronoParseError(chrono::ParseError),
    TimeDoesNotExist,
}

impl std::error::Error for FetchMonthlyEventsError {}
impl std::fmt::Display for FetchMonthlyEventsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchMonthlyEventsError::ChronoParseError(e) => e.fmt(f),
            FetchMonthlyEventsError::TimeDoesNotExist => write!(f, "Time does not exist"),
        }
    }
}

#[async_trait]
pub trait FetchMonthlyEvents {
    async fn monthly_events(&self) -> Result<Vec<PpqEvent>, reqwest::Error>;
}

#[async_trait]
impl FetchMonthlyEvents for super::WikiClient {
    async fn monthly_events(&self) -> Result<Vec<PpqEvent>, reqwest::Error> {
        let url = format!(
            "{}/PPQ:News/{}_{}_Events",
            self.base_url,
            current_month(),
            current_year()
        );
        let event_raw_template = self
            .client
            .get(url)
            .query(&[("action", "raw")])
            .send()
            .await?
            .text()
            .await?;

        Ok(parse_monthly_event_template(&event_raw_template))
    }
}

const MONTH_PROPER_NAMES: [&'static str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

fn current_time_jp() -> DateTime<Tz> {
    Japan.from_utc_datetime(&Utc::now().naive_utc())
}

fn current_month() -> &'static str {
    let time = current_time_jp();
    let month_idx = time.month0();
    let idx: usize = usize::try_from(month_idx).unwrap();
    let name = MONTH_PROPER_NAMES[idx];
    name
}

fn current_year() -> String {
    let time = current_time_jp();
    let year = time.year();
    format!("{year}")
}

fn parse_wiki_time(datetime: &str) -> Option<DateTime<Tz>> {
    let mapped_local_time = NaiveDateTime::parse_from_str(datetime, "%Y/%m/%d %H:%M")
        .ok()?
        .and_local_timezone(Japan);

    match mapped_local_time {
        chrono::offset::LocalResult::Single(t) => Some(t),
        chrono::offset::LocalResult::Ambiguous(e, _) => Some(e),
        chrono::offset::LocalResult::None => None,
    }
}

pub static RE_NO_INCLUDE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<noinclude>.*?</noinclude>").unwrap());

#[derive(Debug, PartialEq, Eq)]
pub enum PpqEventType {
    GuildRush,
    BingoArena,
    LimitedStory,
    Collection,
    Tournament,
    StoryQuest,
    Intrusion,
    Hunting,
    Treasure,
    Unknown,
}

impl From<&str> for PpqEventType {
    fn from(value: &str) -> Self {
        match value {
            "gr" => PpqEventType::GuildRush,
            "ba" => PpqEventType::BingoArena,
            "ls" => PpqEventType::LimitedStory,
            "cl" => PpqEventType::Collection,
            "put" => PpqEventType::Tournament,
            "sq" => PpqEventType::StoryQuest,
            "int" => PpqEventType::Intrusion,
            "hunt" => PpqEventType::Hunting,
            "thb" => PpqEventType::Treasure,
            _ => PpqEventType::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PpqEvent {
    pub icon: String,
    pub r#type: PpqEventType,
    pub start: Option<DateTime<Tz>>,
    pub end: Option<DateTime<Tz>>,
    pub name: Option<String>,
    pub link: Option<String>,
    pub jp_name: Option<String>,
}

fn parse_monthly_event_template(template: &str) -> Vec<PpqEvent> {
    let template = RE_NO_INCLUDE.replace_all(template, "");
    let template = template.trim();

    let events: Vec<PpqEvent> = template
        .split("{{PPQ news/event")
        .filter(|s| s.len() > 0)
        .map(|s| format!("{{{{PPQ news/event{s}").trim().to_string())
        .map(|wiki_text| parse_template(&wiki_text).ok())
        .flatten()
        .map(|value| {
            let name = value
                .get("name")
                .map(|n| n.as_str())
                .flatten()
                .map(|n| n.to_string());
            let link = value
                .get("name")
                .map(|n| n.as_str())
                .flatten()
                .map(|l| match l.trim() {
                    "*" => name.clone(),
                    "--" => None,
                    _ => {
                        if l.len() > 0 {
                            Some(l.to_string())
                        } else {
                            None
                        }
                    }
                })
                .flatten();

            PpqEvent {
                icon: value
                    .get("icon")
                    .map(|i| i.as_str())
                    .flatten()
                    .map(|i| i.trim().to_string())
                    .unwrap_or(String::from("000000")),
                r#type: match value.get("type") {
                    Some(t) => match t.as_str() {
                        Some(t) => PpqEventType::from(t),
                        None => PpqEventType::Unknown,
                    },
                    None => PpqEventType::Unknown,
                },
                start: value
                    .get("start")
                    .map(|s| s.as_str())
                    .flatten()
                    .map(|s| parse_wiki_time(s.trim()))
                    .flatten(),
                end: value
                    .get("end")
                    .map(|e| e.as_str())
                    .flatten()
                    .map(|e| parse_wiki_time(e.trim()))
                    .flatten(),
                name,
                link,
                jp_name: value
                    .get("jpname")
                    .map(|n| n.as_str())
                    .flatten()
                    .map(|n| n.to_string()),
            }
        })
        .collect();

    events
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;

    use super::*;

    #[test]
    #[ignore]
    fn gets_current_time_in_japan() {
        let time = current_time_jp();
        let month_idx = time.month0();
        let day = time.day();
        let name = MONTH_PROPER_NAMES[month_idx as usize];
        assert_eq!(name, "January");
        assert_eq!(day, 20);
    }

    #[test]
    #[ignore]
    fn parses_wiki_time() {
        let time = "2025/01/19 08:00";
        let result = parse_wiki_time(time).unwrap();
        let local_time = result.with_timezone(&chrono_tz::America::New_York);
        println!("{:?}", result);
        println!("{:?}", local_time);
    }

    #[test]
    fn parses_monthly_event_template() {
        let template = r#"<noinclude>{{PPQ news/header|2025|01}}</noinclude>
{{PPQ news/event|icon=000000|type=gr
   |start=2025/02/19 15:00|end=2025/02/23 23:59
   |name=Guild Rush|link=--
   |jpname=ギルドイベント
}}
{{PPQ news/event|icon=549007|type=ls
   |start=2025/01/16 15:00|end=2025/01/29 23:59
   |name=PreCure Series Collab World
   |link=Story/PreCure Series Collab World
   |jpname=『プリキュア』シリーズ コラボの世界
}}
{{PPQ news/event|icon=149007|type=cl
   |start=2025/01/16 15:00|end=2025/01/29 23:59
   |name=Strawberry Melonpan Festival|link=*
   |jpname=いちごメロンパン収集祭り
}}"#;
        let result = parse_monthly_event_template(template);
        for r in result {
            println!("{:?}", r);
        }
    }
}
