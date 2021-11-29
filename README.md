# tinkoff-bank-rs

Some parts of API used by Tinkoff mobile app.

Implementation is very rough, so PRs and issues are welcome.

[![Crates](https://img.shields.io/crates/v/tinkoff-bank)](https://crates.io/crates/tinkoff-bank)
[![API Docs](https://docs.rs/tinkoff-bank/badge.svg)](https://docs.rs/tinkoff-bank)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Basic usage

```rust
use chrono::{DateTime, Utc};
use std::io;
use tinkoff_bank::Client;
use uuid::Uuid;


#[tokio::main]
async fn main() {
    let client = Client::default();

    // Generate random device id
    let device_id = Uuid::new_v4().to_string();

    // Request new session
    let session = client.request_session(&device_id).await;
    let session_id = session.payload.unwrap().id;

    // Start auth by phone
    let phone = input("Enter phone number, like +79998887766: ");
    let confirmation_details = client.auth_by_phone(&device_id, &session_id, &phone).await;
    let operation_ticket = confirmation_details.operation_ticket.unwrap();

    // Confirm auth by phone
    let sms_code = input("Enter the code from sms: ");
    client.confirm_auth_by_phone(&device_id, &session_id, &operation_ticket, &sms_code).await;

    // Auth by password too, it is required to get full access
    let password = input("Enter your password: ");
    client.auth_by_password(&device_id, &session_id, &password).await;

    // List accounts
    let accounts = client.list_accounts(&device_id, &session_id).await
        .payload
        .unwrap();

    // List operations
    let account = &accounts[0];
    client
        .list_operations(
            &device_id,
            &session_id,
            &account.id,
            "2021-02-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            "2021-02-28T00:00:00Z".parse::<DateTime<Utc>>().unwrap()
        )
        .await
}

fn input(text: &str) -> String {
    println!("{}", text);

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_owned()
}
```
