use std::env;

use azure_communications::{types::Recipient, AzureCommunicationService};

#[tokio::main]
async fn main() {
    let connection_string = env::var("AZURE_COMMUNICATIONS_CONNECTION_STRING")
        .expect("Missing AZURE_COMMUNICATIONS_CONNECTION_STRING");

    let sender_adress = env::var("SENDER_ADDRESS").expect("Missing SENDER_ADDRESS");

    let recipients = vec![Recipient {
        address: env::var("RECIPIENT_ADDRESS").expect("Missing RECIPIENT_ADDRESS"),
        display_name: None,
    }];

    let az_communications = AzureCommunicationService::new(&connection_string, None);

    az_communications
        .send_mail(
            &sender_adress,
            "Hello from Azure Communications",
            Some("Hello!"),
            None,
            recipients,
        )
        .await
        .expect("Error sending email");
}
