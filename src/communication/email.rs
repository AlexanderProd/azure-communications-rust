use anyhow::Result;
use reqwest::Url;
use serde::Serialize;
use serde_json::json;

use super::{AzureCommunicationService, AZURE_API_VERSION};

#[derive(Serialize)]
pub struct Recipient {
    pub address: String,
    pub display_name: Option<String>,
}

impl AzureCommunicationService {
    pub async fn send_mail(
        &self,
        sender_address: &str,
        subject: &str,
        plain_text_body: Option<&str>,
        html_body: Option<&str>,
        recipients: Vec<Recipient>,
    ) -> Result<()> {
        let endpoint = &self.endpoint;

        let url = Url::parse(&format!(
            "{}emails:send?api-version={}",
            endpoint, AZURE_API_VERSION
        ))?;

        let body = json!({
            "senderAddress": sender_address,
            "recipients": {
                "to": recipients.iter().map(|r| {
                    json!({
                        "address": r.address,
                        "displayName": r.display_name,
                    })
                }).collect::<Vec<_>>(),
            },
            "content": {
                "subject": subject,
                "plainText": plain_text_body,
                "html": html_body,
            }
        })
        .to_string();

        let request = self.create_az_request(url, body)?;

        match request.send().await {
            Ok(response) => match response.status().as_u16() {
                200..=299 => Ok(()),
                _ => {
                    let body = response.text().await?;
                    Err(anyhow::anyhow!("Error sending email: {}", body))
                }
            },
            Err(e) => Err(anyhow::Error::from(e)),
        }
    }
}
