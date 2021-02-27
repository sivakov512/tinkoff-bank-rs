use httpmock::MockServer;
use rstest::*;
use tinkoff_bank_rs::{Client, ClientBuilder, Nothing, ResponsePayload, ResultCode};

const RESPONSE: &str = "{\"payload\": {\"key\": \"key-example\"}, \"resultCode\": \"OK\", \"trackingId\": \"AZAZA11\"}";

#[fixture]
fn server() -> MockServer {
    MockServer::start()
}

fn make_client(server: &MockServer) -> Client {
    ClientBuilder::default()
        .with_url(&server.base_url())
        .build()
}

#[rstest]
#[tokio::test]
async fn returns_nothing(server: MockServer) {
    server.mock(|when, then| {
        when.method(httpmock::Method::POST).path("/v1/auth/pin/set");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    let got = make_client(&server)
        .set_auth_pin("ultra-session-id", "ultra-hash")
        .await;

    assert_eq!(
        got,
        ResponsePayload {
            result_code: ResultCode::Ok,
            payload: Some(Nothing {}),
            confirmations: None,
            initial_operation: None,
            operation_ticket: None,
        }
    )
}

#[rstest]
#[tokio::test]
async fn passes_session_id_and_params(server: MockServer) {
    let mock = server.mock(|when, then| {
        when.method(httpmock::Method::POST)
            .path("/v1/auth/pin/set")
            .query_param("sessionid", "ultra-session-id")
            .body("pinHash=ultra-hash");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    make_client(&server)
        .set_auth_pin("ultra-session-id", "ultra-hash")
        .await;

    mock.assert()
}