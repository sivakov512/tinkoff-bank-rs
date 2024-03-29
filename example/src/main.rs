use chrono::{DateTime, Utc};
use std::io;
use tinkoff_bank::Client;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let device_id = Uuid::new_v4().to_string();
    let client = Client::default();

    print_section("Request session");
    let session = dbg!(client.request_session(&device_id).await);
    let session_id = session.payload.unwrap().id;
    dbg!(client.ping(&device_id, &session_id).await);

    print_section("Auth by phone");
    let phone = input("Enter phone number, like +79998887766: ");
    let confirmation_details = dbg!(client.auth_by_phone(&device_id, &session_id, &phone).await);
    let operation_ticket = confirmation_details.operation_ticket.unwrap();

    print_section("Confirm auth by phone");
    let sms_code = input("Enter the code from sms: ");
    dbg!(
        client
            .confirm_auth_by_phone(&device_id, &session_id, &operation_ticket, &sms_code)
            .await
    );
    dbg!(client.ping(&device_id, &session_id).await);

    print_section("Auth by password");
    let password = input("Enter your password: ");
    dbg!(
        client
            .auth_by_password(&device_id, &session_id, &password)
            .await
    );
    dbg!(client.ping(&device_id, &session_id).await);

    print_section("Set auth pin");
    let auth_pin = Uuid::new_v4().to_string();
    dbg!(
        client
            .set_auth_pin(&device_id, &session_id, &auth_pin)
            .await
    );

    print_section("Auth by pin");
    let new_session_id = dbg!(client.request_session(&device_id,).await)
        .payload
        .unwrap()
        .id;
    dbg!(
        client
            .auth_by_pin(&device_id, &new_session_id, &auth_pin, &session_id)
            .await
    );
    dbg!(client.ping(&device_id, &new_session_id).await);

    print_section("List accounts");
    let accounts = dbg!(client.list_accounts(&device_id, &new_session_id).await)
        .payload
        .unwrap();
    let account = &accounts[0];

    print_section("List operations");
    dbg!(
        client
            .list_operations(
                &device_id,
                &new_session_id,
                &account.id,
                dt("2021-02-01T00:00:00Z"),
                dt("2021-02-28T00:00:00Z")
            )
            .await
    );
}

fn input(text: &str) -> String {
    println!("{}", text);

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_owned()
}

fn print_section(text: &str) {
    println!("\n");
    println!("{}\n======", text);
}

fn dt(value: &str) -> DateTime<Utc> {
    value.parse::<DateTime<Utc>>().unwrap()
}
