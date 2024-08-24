use std::env;

use azure_communications::AzureCommunicationService;

#[tokio::main]
async fn main() {
    let connection_string = env::var("AZURE_COMMUNICATIONS_CONNECTION_STRING")
        .expect("Missing AZURE_COMMUNICATIONS_CONNECTION_STRING");

    let sender_name = env::var("SENDER_NAME").expect("Missing SENDER_NAME");

    let recipient_phone_number =
        env::var("RECIPIENT_PHONE_NUMBER").expect("Missing RECIPIENT_PHONE_NUMBER");

    let az_communications = AzureCommunicationService::new(&connection_string, None);

    az_communications
        .send_sms(
            &sender_name,
            "Hello World sent by SMS. 👋",
            vec![&recipient_phone_number],
        )
        .await
        .expect("Error sending email");
}
