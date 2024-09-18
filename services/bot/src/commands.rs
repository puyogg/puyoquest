pub mod char_by_id;
pub mod card;

#[allow(unused)]
pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
pub type Context<'a> = poise::Context<'a, Data, Error>;

// Data which is stored and accessible in all command invocations
#[derive(Debug)]
pub struct Data {
    pub hello: String,
}
