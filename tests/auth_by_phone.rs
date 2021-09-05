use httpmock::MockServer;
use rstest::*;
use tinkoff_bank::{Client, ClientBuilder, ResponsePayload, ResultCode};

const RESPONSE: &str = "{\"confirmationData\": {\"SMSBYID\": {\"codeLength\": 4, \"codeType\": \"Numeric\", \"confirmationType\": \"SMSBYID\"}}, \"confirmations\": [\"SMSBYID\"], \"initialOperation\": \"auth/by/phone\", \"operationTicket\": \"operation-ticket-example\", \"resultCode\": \"WAITING_CONFIRMATION\", \"trackingId\": \"AZAZA11\"}";

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
async fn returns_confirmation_details(server: MockServer) {
    server.mock(|when, then| {
        when.method(httpmock::Method::POST)
            .path("/v1/auth/by/phone");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    let got = make_client(&server)
        .auth_by_phone("ultra-device-id", "ultra-session-id", "+79991112233")
        .await;

    assert_eq!(
        got,
        ResponsePayload {
            result_code: ResultCode::WaitingConfirmation,
            payload: None,
            confirmations: Some(vec!["SMSBYID".to_owned()]),
            initial_operation: Some("auth/by/phone".to_owned()),
            operation_ticket: Some("operation-ticket-example".to_owned()),
        }
    )
}

#[rstest]
#[tokio::test]
async fn passes_params(server: MockServer) {
    let mock = server.mock(|when, then| {
        when.method(httpmock::Method::POST)
            .path("/v1/auth/by/phone")
            .query_param("sessionid", "ultra-session-id")
            .query_param("deviceId", "ultra-device-id")
            .body("phone=%2B79991112233");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    make_client(&server)
        .auth_by_phone("ultra-device-id", "ultra-session-id", "+79991112233")
        .await;

    mock.assert()
}
