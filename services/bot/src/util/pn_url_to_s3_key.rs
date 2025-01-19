use url::Url;

pub fn pn_url_to_s3_key(url: &str) -> Option<String> {
    Url::parse(url).map(|u| u.path()[1..].to_string()).ok()
}
