use crate::cards::template_data::CardTemplateData;
use wiki::wiki_client::{ResolveWikiText, ResolveWikiTextError, WikiClient};

async fn resolve_wiki_text(
    wiki_client: &WikiClient,
    text: &Option<String>,
) -> Result<Option<String>, ResolveWikiTextError> {
    match text {
        None => Ok(None),
        Some(t) => {
            let t = t.as_str();
            let res = wiki_client.resolve_wiki_text(t).await?;

            Ok(Some(res.parse.text))
        }
    }
}

pub async fn resolve_card_template(
    wiki_client: &WikiClient,
    template_data: CardTemplateData,
) -> Result<CardTemplateData, ResolveWikiTextError> {
    let (bse, sse, cae, tse) = futures::try_join!(
        resolve_wiki_text(&wiki_client, &template_data.bse),
        resolve_wiki_text(&wiki_client, &template_data.sse),
        resolve_wiki_text(&wiki_client, &template_data.cae),
        resolve_wiki_text(&wiki_client, &template_data.tse),
    )?;

    let (ase, asfe, lse) = futures::try_join!(
        resolve_wiki_text(&wiki_client, &template_data.ase),
        resolve_wiki_text(&wiki_client, &template_data.asfe),
        resolve_wiki_text(&wiki_client, &template_data.lse),
    )?;

    let (aste, ast2e, ast3e) = futures::try_join!(
        resolve_wiki_text(&wiki_client, &template_data.aste),
        resolve_wiki_text(&wiki_client, &template_data.ast2e),
        resolve_wiki_text(&wiki_client, &template_data.ast3e),
    )?;

    let (lste, lst2e, lst3e) = futures::try_join!(
        resolve_wiki_text(&wiki_client, &template_data.lste),
        resolve_wiki_text(&wiki_client, &template_data.lst2e),
        resolve_wiki_text(&wiki_client, &template_data.lst3e),
    )?;

    let resolved_template = CardTemplateData {
        ase,
        asfe,
        aste,
        ast2e,
        ast3e,
        lse,
        lste,
        lst2e,
        lst3e,
        bse,
        sse,
        cae,
        tse,
        ..template_data
    };

    Ok(resolved_template)
}
