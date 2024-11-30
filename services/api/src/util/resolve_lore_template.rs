use crate::cards::lore::WikiLore;
use wiki::wiki_client::{ResolveWikiTextError, WikiClient};

use super::resolve_card_template::resolve_wiki_text;

pub async fn resolve_lore_template(
    wiki_client: &WikiClient,
    lore: WikiLore,
) -> Result<WikiLore, ResolveWikiTextError> {
    let (ft, fta, ftc, ft1, fta1) = futures::try_join!(
        resolve_wiki_text(&wiki_client, &lore.ft),
        resolve_wiki_text(&wiki_client, &lore.fta),
        resolve_wiki_text(&wiki_client, &lore.ftc),
        resolve_wiki_text(&wiki_client, &lore.ft1),
        resolve_wiki_text(&wiki_client, &lore.fta1),
    )?;

    let (ft2, fta2, ft3, fta3) = futures::try_join!(
        resolve_wiki_text(&wiki_client, &lore.ft2),
        resolve_wiki_text(&wiki_client, &lore.fta2),
        resolve_wiki_text(&wiki_client, &lore.ft3),
        resolve_wiki_text(&wiki_client, &lore.fta3),
    )?;

    Ok(WikiLore {
        ft,
        fta,
        ftc,
        ft1,
        fta1,
        ft2,
        fta2,
        ft3,
        fta3,
        ..lore
    })
}
