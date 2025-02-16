#[derive(Debug)]
pub enum ParsingError {
    RegexError(fancy_regex::Error)
}

impl std::error::Error for ParsingError {}
impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::RegexError(error) => error.fmt(f),
        }
    }
}

impl From<fancy_regex::Error> for ParsingError {
    fn from(value: fancy_regex::Error) -> Self {
        ParsingError::RegexError(value)
    }
}
