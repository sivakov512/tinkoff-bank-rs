use httpmock::MockServer;
use rstest::*;
use tinkoff_bank_rs::{AccessLevel, Client, ClientBuilder, ResponsePayload, ResultCode, UserInfo};

const RESPONSE: &str = "{\"payload\": {\"accessLevel\": \"CANDIDATE\", \"firstName\": \"Cool guy\", \"hasPassword\": true, \"key\": \"key-example\", \"noClient\": false, \"ssoId\": \"sso-id-example\", \"userId\": \"user-id-example\"}, \"resultCode\": \"OK\", \"trackingId\": \"AZAZA11\"}";

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
async fn returns_user_info(server: MockServer) {
    server.mock(|when, then| {
        when.method(httpmock::Method::POST).path("/v1/confirm");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    let got = make_client(&server)
        .confirm_auth_by_phone("ultra-session-id", "ultra-operation-ticket", "1234")
        .await;

    assert_eq!(
        got,
        ResponsePayload {
            result_code: ResultCode::Ok,
            payload: Some(UserInfo {
                access_level: AccessLevel::Candidate,
                user_id: "user-id-example".to_owned()
            }),
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
            .path("/v1/confirm")
            .query_param("sessionid", "ultra-session-id")
            .body("initialOperationTicket=ultra-operation-ticket&initialOperation=auth%2Fby%2Fphone&confirmationData=%7B%22SMSBYID%22%3A%221234%22%7D");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    make_client(&server)
        .confirm_auth_by_phone("ultra-session-id", "ultra-operation-ticket", "1234")
        .await;

    mock.assert()
}
