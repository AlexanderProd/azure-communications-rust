use crate::utils::{create_authorization_signature, get_utc_string};
use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as base64, Engine};
use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client, Url,
};
use ring::digest::{digest, SHA256};
use time::OffsetDateTime;

pub mod email;
pub mod sms;

const AZURE_API_VERSION: &str = "2023-03-31";

#[derive(Debug, Clone)]
pub struct AzureCommunicationService {
    pub endpoint: String,
    pub access_key: String,
    pub request_client: Client,
}

impl AzureCommunicationService {
    pub fn new(connection_string: &str, request_client: Option<Client>) -> Self {
        let re = Regex::new(r"endpoint=(.*?);accesskey=(.*)").expect("Invalid regex");
        let caps = re
            .captures(connection_string)
            .expect("Invalid connection string");

        let endpoint = caps[1].to_string();
        let access_key = caps[2].to_string();
        let request_client = request_client.unwrap_or_default();

        AzureCommunicationService {
            endpoint: endpoint.to_string(),
            access_key: access_key.to_string(),
            request_client,
        }
    }
}

impl AzureCommunicationService {
    fn create_az_request(&self, url: Url, body: String) -> Result<reqwest::RequestBuilder> {
        let access_key = &self.access_key;
        let client = &self.request_client;

        let content_hash = digest(&SHA256, body.as_bytes());
        let content_hash_base64 = base64.encode(content_hash);

        let now = OffsetDateTime::now_utc();
        let date = get_utc_string(now)?;

        let signature = create_authorization_signature(&url, &body, access_key, &date)?;

        let mut headers = HeaderMap::new();
        headers.insert("x-ms-date", HeaderValue::from_str(&date)?);
        headers.insert(
            "x-ms-content-sha256",
            HeaderValue::from_str(&content_hash_base64)?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "HMAC-SHA256 SignedHeaders=x-ms-date;host;x-ms-content-sha256&Signature={}",
                signature
            ))?,
        );

        let request = client.post(url).headers(headers).body(body);

        Ok(request)
    }
}
