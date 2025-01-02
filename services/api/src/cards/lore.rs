use fancy_regex::Regex;
use poem::Result;
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse, Object,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use wiki::wiki_client::WikiClient;

use crate::cache::card_lore_data;
use crate::cache::RedisClient;
use crate::cards::get_by_id::query_get_by_id;

#[derive(Debug, Clone, Object, Serialize, Deserialize, PartialEq, Eq)]
pub struct WikiLore {
    pub code: String,
    /// JP description
    pub ft: Option<String>,
    /// English description
    pub fta: Option<String>,
    /// Translator
    pub ftc: Option<String>,
    pub ft1: Option<String>,
    pub fta1: Option<String>,
    pub ft2: Option<String>,
    pub fta2: Option<String>,
    pub ft3: Option<String>,
    pub fta3: Option<String>,
}

#[derive(Debug, Clone, Object, Serialize, Deserialize, PartialEq, Eq)]
pub struct MonologueLine {
    pub jp: Option<String>,
    pub en: Option<String>,
}

#[derive(Debug, Clone, Object, Serialize, Deserialize, PartialEq, Eq)]
pub struct Lore {
    pub card_id: String,
    pub flavor_text_jp: Option<String>,
    pub flavor_text_en: Option<String>,
    pub monologue_lines: Vec<MonologueLine>,
    pub translator: Option<String>,
    pub editor: Option<String>,
}

impl Lore {
    pub fn has_a_translation(&self) -> bool {
        if self.flavor_text_en.is_some() {
            return true;
        }

        for monologue_line in &self.monologue_lines {
            if monologue_line.en.is_some() {
                return true;
            }
        }

        return false;
    }
}

lazy_static::lazy_static! {
    static ref RE_CITATION: Regex = Regex::new(r"(?ms)\[\d\]").unwrap();
    static ref RE_BRACKETS: Regex = Regex::new(r"[\[\]]").unwrap();
    static ref RE_TRANSLATORS: Regex = Regex::new(r"(?ms)Translator\n(^.*?$)").unwrap();
    static ref RE_EDITORS: Regex = Regex::new(r"(?ms)Editor\n(^.*?$)").unwrap();
}

impl From<WikiLore> for Lore {
    fn from(value: WikiLore) -> Self {
        let mut monologue_lines: Vec<MonologueLine> = Vec::new();
        for (jp, en) in &[
            (value.ft1, value.fta1),
            (value.ft2, value.fta2),
            (value.ft3, value.fta3),
        ] {
            if let (None, None) = (jp, en) {
                continue;
            } else {
                monologue_lines.push(MonologueLine {
                    jp: jp.clone(),
                    en: en.clone(),
                })
            }
        }

        let (translator, editor) = match value.ftc {
            None => (None, None),
            Some(ftc) => {
                let translator_captures = RE_TRANSLATORS.captures_iter(&ftc);
                let translators = translator_captures
                    .flat_map(|c| match c {
                        Err(_) => None,
                        Ok(c) => {
                            let translator = c.get(1);
                            match translator {
                                None => None,
                                Some(t) => {
                                    let t = t.as_str();
                                    let t = RE_CITATION.replace_all(&t, "");
                                    let t = RE_BRACKETS.replace_all(&t, "");
                                    let t = t.trim();

                                    if t == "None" {
                                        None
                                    } else {
                                        Some(t.to_string())
                                    }
                                }
                            }
                        }
                    })
                    .collect::<Vec<String>>();

                let editor_captures = RE_EDITORS.captures_iter(&ftc);
                let editors = editor_captures
                    .flat_map(|c| match c {
                        Err(_) => None,
                        Ok(c) => {
                            let editor = c.get(1);
                            match editor {
                                None => None,
                                Some(e) => {
                                    let e = e.as_str();
                                    let e = RE_CITATION.replace_all(&e, "");
                                    let e = RE_BRACKETS.replace_all(&e, "");
                                    let e = e.trim();

                                    if e == "None" {
                                        None
                                    } else {
                                        Some(e.to_string())
                                    }
                                }
                            }
                        }
                    })
                    .collect::<Vec<String>>();

                let t = if translators.len() > 0 {
                    Some(translators.join(", "))
                } else {
                    None
                };
                let e = if editors.len() > 0 {
                    Some(editors.join(", "))
                } else {
                    None
                };
                (t, e)
            }
        };

        Self {
            card_id: value.code,
            flavor_text_jp: value.ft,
            flavor_text_en: value.fta,
            monologue_lines,
            translator,
            editor,
        }
    }
}

#[derive(ApiResponse)]
pub enum GetCardLoreResponse {
    #[oai(status = 200)]
    Lore(Json<Lore>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

pub async fn get_card_lore(
    pool: &PgPool,
    redis_client: &RedisClient,
    wiki_client: &WikiClient,
    card_id: &str,
) -> Result<GetCardLoreResponse> {
    // Validate the card_id
    let card_db = query_get_by_id(pool, card_id).await?;
    if let None = card_db {
        return Ok(GetCardLoreResponse::NotFound(PlainText(format!(
            "card_id not found: {}",
            card_id
        ))));
    }

    let wiki_lore = card_lore_data(redis_client, wiki_client, card_id).await?;
    let lore = Lore::from(wiki_lore);

    Ok(GetCardLoreResponse::Lore(Json(lore)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translator_and_editor() {
        // 402207
        let wiki_lore = WikiLore {
            code: "402207".to_string(),
            ft: Some("さまざまな世界を旅して まわっている「時空の旅人」。 怒らせるとちょっとコワイ。".to_string()),
            fta: Some("A \"space-time traveler\" who travels around various worlds. He becomes a bit scary when agitated.".to_string()),
            ftc: Some("Translator\n[Beachedking][1]\nEditor\n[Pi][2]\n\n[1]: /wiki/User:Beachedking\n[2]: /wiki/User:Pi".to_string()),
            ft1: Some("あ、いたの？ 気付かなかった".to_string()),
            fta1: Some("Ah, you were there? I didn't notice.".to_string()),
            ft2: Some("いろんなところを旅してきたけど ここもひかくてき楽しいね".to_string()),
            fta2: Some("I've traveled to lots of different places, but coming here is also pretty fun!".to_string()),
            ft3: Some("とってもいい気分だよ くすくすくすくす…".to_string()),
            fta3: Some("I feel great! *chuckle*".to_string()),
        };

        let expected_lore = Lore {
            card_id: "402207".to_string(),
            flavor_text_jp: wiki_lore.ft.clone(),
            flavor_text_en: wiki_lore.fta.clone(),
            monologue_lines: vec![
                MonologueLine {
                    jp: wiki_lore.ft1.clone(),
                    en: wiki_lore.fta1.clone(),
                },
                MonologueLine {
                    jp: wiki_lore.ft2.clone(),
                    en: wiki_lore.fta2.clone(),
                },
                MonologueLine {
                    jp: wiki_lore.ft3.clone(),
                    en: wiki_lore.fta3.clone(),
                },
            ],
            translator: Some("Beachedking".to_string()),
            editor: Some("Pi".to_string()),
        };

        let lore = Lore::from(wiki_lore);
        assert_eq!(lore, expected_lore);
    }

    #[test]
    fn translator_only() {
        let wiki_lore = WikiLore {
            code: "402207".to_string(),
            ft: Some("さまざまな世界を旅して まわっている「時空の旅人」。 怒らせるとちょっとコワイ。".to_string()),
            fta: Some("A \"space-time traveler\" who travels around various worlds. He becomes a bit scary when agitated.".to_string()),
            ftc: Some("Translator\n[Beachedking][1]\nEditor\nNone\n\n[1]: /wiki/User:Beachedking".to_string()),
            ft1: Some("あ、いたの？ 気付かなかった".to_string()),
            fta1: Some("Ah, you were there? I didn't notice.".to_string()),
            ft2: Some("いろんなところを旅してきたけど ここもひかくてき楽しいね".to_string()),
            fta2: Some("I've traveled to lots of different places, but coming here is also pretty fun!".to_string()),
            ft3: Some("とってもいい気分だよ くすくすくすくす…".to_string()),
            fta3: Some("I feel great! *chuckle*".to_string()),
        };

        let expected_lore = Lore {
            card_id: "402207".to_string(),
            flavor_text_jp: wiki_lore.ft.clone(),
            flavor_text_en: wiki_lore.fta.clone(),
            monologue_lines: vec![
                MonologueLine {
                    jp: wiki_lore.ft1.clone(),
                    en: wiki_lore.fta1.clone(),
                },
                MonologueLine {
                    jp: wiki_lore.ft2.clone(),
                    en: wiki_lore.fta2.clone(),
                },
                MonologueLine {
                    jp: wiki_lore.ft3.clone(),
                    en: wiki_lore.fta3.clone(),
                },
            ],
            translator: Some("Beachedking".to_string()),
            editor: None,
        };

        let lore = Lore::from(wiki_lore);
        assert_eq!(lore, expected_lore);
    }

    #[test]
    fn no_translator() {
        let wiki_lore = WikiLore {
            code: "402207".to_string(),
            ft: Some("さまざまな世界を旅して まわっている「時空の旅人」。 怒らせるとちょっとコワイ。".to_string()),
            fta: Some("A \"space-time traveler\" who travels around various worlds. He becomes a bit scary when agitated.".to_string()),
            ftc: None,
            ft1: Some("あ、いたの？ 気付かなかった".to_string()),
            fta1: Some("Ah, you were there? I didn't notice.".to_string()),
            ft2: Some("いろんなところを旅してきたけど ここもひかくてき楽しいね".to_string()),
            fta2: Some("I've traveled to lots of different places, but coming here is also pretty fun!".to_string()),
            ft3: Some("とってもいい気分だよ くすくすくすくす…".to_string()),
            fta3: Some("I feel great! *chuckle*".to_string()),
        };

        let expected_lore = Lore {
            card_id: "402207".to_string(),
            flavor_text_jp: wiki_lore.ft.clone(),
            flavor_text_en: wiki_lore.fta.clone(),
            monologue_lines: vec![
                MonologueLine {
                    jp: wiki_lore.ft1.clone(),
                    en: wiki_lore.fta1.clone(),
                },
                MonologueLine {
                    jp: wiki_lore.ft2.clone(),
                    en: wiki_lore.fta2.clone(),
                },
                MonologueLine {
                    jp: wiki_lore.ft3.clone(),
                    en: wiki_lore.fta3.clone(),
                },
            ],
            translator: None,
            editor: None,
        };

        let lore = Lore::from(wiki_lore);
        assert_eq!(lore, expected_lore);
    }
}
