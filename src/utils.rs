use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as base64, Engine};
use lazy_static::lazy_static;
use reqwest::Url;
use ring::{
    digest::{digest, SHA256},
    hmac,
};
use time::{format_description, OffsetDateTime};

pub fn create_authorization_signature(
    endpoint: &Url,
    request_body: &str,
    access_key: &str,
    date: &str,
) -> Result<String> {
    let uri_path_and_query = if let Some(query) = endpoint.query() {
        format!("{}?{}", endpoint.path(), query)
    } else {
        endpoint.path().to_string()
    };

    let host = endpoint
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid host"))?;

    let content_hash = digest(&SHA256, request_body.as_bytes());
    let content_hash_base64 = base64.encode(content_hash.as_ref());

    let decoded_key = base64.decode(access_key)?;

    let string_to_sign = format!(
        "POST\n{}\n{};{};{}",
        uri_path_and_query, date, host, content_hash_base64
    );

    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, &decoded_key);
    let signature = hmac::sign(&signed_key, string_to_sign.as_bytes());

    Ok(base64.encode(signature))
}

pub fn format_utc_string(timestamp: OffsetDateTime) -> Result<String> {
    lazy_static! {
        static ref RFC1123: Vec<format_description::FormatItem<'static>> = format_description::parse(
            "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT",
        )
        .expect("Failed to parse format description for RFC1123");
    }

    let formatted = timestamp.format(&RFC1123)?;

    Ok(formatted)
}
