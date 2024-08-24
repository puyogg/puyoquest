use async_trait::async_trait;
use chrono::{DateTime, Utc};
use fancy_regex::{Match, Regex};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize)]
pub struct RecentChangesResponse {
    pub r#continue: Option<RecentChangesContinue>,
    pub query: RecentChangesQuery,
}

#[derive(Deserialize)]
pub struct RecentChangesContinue {
    pub rccontinue: String,
    pub r#continue: String,
}

#[derive(Deserialize)]
pub struct RecentChangesQuery {
    pub recentchanges: Vec<RecentChangeItem>,
}

#[derive(Deserialize)]
pub struct RecentChangeItem {
    pub title: String,
}

#[async_trait]
pub trait RecentCharChanges {
    async fn recent_char_changes(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<HashSet<String>, reqwest::Error>;
}

lazy_static! {
    static ref CARD_OR_CHAR_ID: Regex = Regex::new(r"Template:(\d{4,6})$").unwrap();
}

#[async_trait]
impl RecentCharChanges for super::WikiClient {
    async fn recent_char_changes(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<HashSet<String>, reqwest::Error> {
        let mut char_ids: HashSet<String> = HashSet::new();

        let mut has_continue = true;
        let mut r#continue: Option<String> = None;
        let mut rccontinue: Option<String> = None;

        let rcstart = start.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let rcend = end.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

        let mut iter = 0;
        while has_continue {
            // println!("{}", iter);
            iter += 1;
            // failsafe to make sure this never goes infinitely
            // With 500 per page, it shouldn't get this high.
            if iter >= 20 {
                println!("broken");
                break;
            }

            let mut query_params = vec![
                ("action", "query"),
                ("format", "json"),
                ("list", "recentchanges"),
                ("formatversion", "2"),
                ("rcstart", &rcstart),
                ("rcend", &rcend),
                ("rcdir", "newer"),
                ("rcnamespace", "10"), // "Template:" namespace
                ("rcprop", "title"),
                ("rclimit", "500"),
            ];

            if r#continue.is_some() && rccontinue.is_some() {
                query_params.push(("continue", r#continue.as_ref().unwrap()));
                query_params.push(("rccontinue", rccontinue.as_ref().unwrap()));
            }

            let response = self
                .client
                .get(&self.api_url)
                .query(&query_params)
                .send()
                .await?;
            println!("{}", response.url());
            let response = response.json::<RecentChangesResponse>().await?;

            for recent_change in response.query.recentchanges {
                let title = recent_change.title;
                let capture_result = CARD_OR_CHAR_ID.captures(&title);

                let capture_match: Option<Match> =
                    capture_result.ok().flatten().and_then(|c| c.get(1));

                match capture_match {
                    Some(m) => {
                        let char_id = &m.as_str()[0..4];
                        char_ids.insert(String::from(char_id));
                    }
                    None => continue,
                };
            }

            match response.r#continue {
                Some(c) => {
                    r#continue = Some(c.r#continue);
                    rccontinue = Some(c.rccontinue);
                }
                None => {
                    has_continue = false;
                }
            }
        }

        Ok(char_ids)
    }
}
