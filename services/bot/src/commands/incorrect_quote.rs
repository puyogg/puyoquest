use super::{Context, Error};

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

    Ok(())
}


