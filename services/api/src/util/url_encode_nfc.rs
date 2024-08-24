use unicode_normalization::UnicodeNormalization;
use urlencoding::encode;

/// I guess the actual URIs should be NFC normalized to match how browsers handle them?
pub fn url_encode_nfc(path: impl AsRef<str>) -> String {
    let path = path.as_ref();
    let encoded = encode(&path.nfc().to_string()).to_string();
    encoded
}
