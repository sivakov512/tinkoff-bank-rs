use httpmock::MockServer;
use rstest::*;
use tinkoff_bank::{AccessLevel, Client, ResponsePayload, ResultCode, UserInfo};

const RESPONSE: &str = "{\"payload\": {\"accessLevel\": \"CANDIDATE\", \"firstName\": \"Cool guy\", \"hasPassword\": true, \"key\": \"key-example\", \"noClient\": false, \"ssoId\": \"sso-id-example\", \"userId\": \"user-id-example\"}, \"resultCode\": \"OK\", \"trackingId\": \"AZAZA11\"}";

#[fixture]
fn server() -> MockServer {
    MockServer::start()
}

fn make_client(server: &MockServer) -> Client {
    Client::new(&server.base_url())
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
        .confirm_auth_by_phone(
            "ultra-device-id",
            "ultra-session-id",
            "ultra-operation-ticket",
            "1234",
        )
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
async fn passes_params(server: MockServer) {
    let mock = server.mock(|when, then| {
        when.method(httpmock::Method::POST)
            .path("/v1/confirm")
            .query_param("sessionid", "ultra-session-id")
            .query_param("deviceId", "ultra-device-id")
            .body("initialOperationTicket=ultra-operation-ticket&initialOperation=auth%2Fby%2Fphone&confirmationData=%7B%22SMSBYID%22%3A%221234%22%7D");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    make_client(&server)
        .confirm_auth_by_phone(
            "ultra-device-id",
            "ultra-session-id",
            "ultra-operation-ticket",
            "1234",
        )
        .await;

    mock.assert()
}
