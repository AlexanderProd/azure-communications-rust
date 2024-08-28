use anyhow::Result;
use reqwest::Url;
use serde_json::json;

use super::AzureCommunicationService;

const API_VERSION: &str = "2021-03-07";

impl AzureCommunicationService {
    /// Message must not exceed 160 characters.
    /// Phone numbers must be in E.164 format.
    pub async fn send_sms(
        &self,
        sender_name: &str,
        message: &str,
        recipient_phone_numbers: Vec<&str>,
    ) -> Result<()> {
        let endpoint = &self.endpoint;

        let url = Url::parse(&format!("{}sms?api-version={}", endpoint, API_VERSION))?;

        let body = json!({
            "from": sender_name,
            "smsRecipients":
               recipient_phone_numbers.iter().map(|r| {
                    json!({
                        "to": r,
                    })
               }).collect::<Vec<_>>(),
            "message": message,
        })
        .to_string();

        let request = self.create_az_request(url, body)?;

        match request.send().await {
            Ok(response) => match response.status().as_u16() {
                200..=299 => Ok(()),
                _ => {
                    let body = response.text().await?;
                    Err(anyhow::anyhow!("Error sending sms: {}", body))
                }
            },
            Err(e) => Err(anyhow::Error::from(e)),
        }
    }
}
