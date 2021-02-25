use std::io;
use tinkoff_bank_rs::ClientBuilder;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::default().build();

    print_section("Request session");
    let session = dbg!(client.request_session().await);
    let session_id = session.payload.unwrap().id;
    dbg!(client.ping(&session_id).await);

    print_section("Auth by phone");
    let phone = input("Enter phone number, like +79998887766: ");
    let confirmation_details = dbg!(client.auth_by_phone(&session_id, &phone).await);
    let operation_ticket = confirmation_details.operation_ticket.unwrap();

    print_section("Confirm auth by phone");
    let sms_code = input("Enter the code from sms: ");
    dbg!(
        client
            .confirm_auth_by_phone(&session_id, &operation_ticket, &sms_code)
            .await
    );
    dbg!(client.ping(&session_id).await);

    print_section("Auth by password");
    let password = input("Enter your password: ");
    dbg!(client.auth_by_password(&session_id, &password).await);
    dbg!(client.ping(&session_id).await);
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
