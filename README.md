# Azure Communication Services for Rust 📱

[![crates.io](https://img.shields.io/crates/v/azure-communications.svg)](https://crates.io/crates/azure-communications)
[![dependency status](https://deps.rs/repo/github/AlexanderProd/azure-communications-rust/status.svg)](https://deps.rs/repo/github/AlexanderProd/azure-communications-rust)
[![charls documentation](https://docs.rs/azure-communications/badge.svg)](https://docs.rs/azure-communications)

This Rust library provides a convenient wrapper for the Azure Communications API, making it easy to integrate Azure Communication Services into your Rust projects.

## Features

- Easy-to-use interface for Azure Communication Services
- Support for sending SMS messages
- Support for sending emails

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
azure-communications = "0.1.1"
```

or run this command in your project directory:

```sh
cargo add azure-communications
```

## 🚀 Example

Make sure to register an account with Azure Communication Services and create a connection string. You can find more information on how to do this [here](https://docs.microsoft.com/en-us/azure/communication-services/quickstarts/create-communication-resource).

To use the example file:

1. Clone this repository
2. Set the required environment variables:
   - On Unix-like systems:
     ```
     export AZURE_COMMUNICATIONS_CONNECTION_STRING="your_connection_string"
     export SENDER_ADDRESS="sender@example.com"
     export RECIPIENT_ADDRESS="recipient@example.com"
     ```
   - On Windows:
     ```
     set AZURE_COMMUNICATIONS_CONNECTION_STRING=your_connection_string
     set SENDER_ADDRESS=sender@example.com
     set RECIPIENT_ADDRESS=recipient@example.com
     ```
3. Run the example: `cargo run --example email`

The program will attempt to send an email using the Azure Communications Service.

## Usage

### Sending an email 📧

The `send_mail` method requires a sender address, a subject, an optional body, an optional HTML body, and a list of recipients. The sender address must be a valid email address and must be registered in the Azure Communication Services portal. More information can be found in the [official documentation](https://learn.microsoft.com/en-us/azure/communication-services/concepts/email/prepare-email-communication-resource).

The recipient list must contain at least one recipient, and each recipient must have a valid email address, as well as an optional display name.

```rust
use azure_communication::{AzureCommunicationsClient, types::Recipient};

let az_communications = AzureCommunicationService::new(&connection_string, None);

let recipients = vec![Recipient {
    address: "test@test.de".to_string(),
    display_name: Some("Test".to_string()),
}];

let sender_adress = "sender@mail.com";

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
```

### Sending an SMS 📱

The `sender_name` argument provided to the `send_sms` method can be either the string of a E.164 formatted phone number or a string of up to 11 alphanumeric characters. The sender phone number needs to be registered in the Azure Communication Services portal. Check the [official documentation](https://learn.microsoft.com/en-us/azure/communication-services/concepts/sms/concepts) for more information.

```rust
use azure_communication::AzureCommunicationsClient;

let az_communications = AzureCommunicationService::new(&connection_string, None);

az_communications
    .send_sms("SampleCoLtd", "Hello from Azure Communications", vec!["+1234567890"])
    .await
    .expect("Error sending SMS");
```

## 📌 ToDo

- [ ] Add support for phone calls
