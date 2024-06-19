use poem_openapi::Tags;

#[derive(Tags)]
pub enum ApiTag {
    Cards,
    Characters,
    Healthcheck,
    Aliases,
}
