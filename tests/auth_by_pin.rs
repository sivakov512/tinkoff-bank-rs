use httpmock::MockServer;
use rstest::*;
use tinkoff_bank::{AccessLevel, Client, ResponsePayload, ResultCode, UserInfo};

const RESPONSE: &str = "{\"resultCode\": \"OK\", \"payload\": {\"key\": \"key-example\", \"deviceId\": \"ultra-device-id\", \"accessLevel\": \"CLIENT\", \"noClient\": false, \"ssoId\": \"sso-id-example\"}, \"trackingId\": \"AZAZA11\"}";

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
        when.method(httpmock::Method::POST).path("/v1/auth/by/pin");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    let got = make_client(&server)
        .auth_by_pin(
            "ultra-device-id",
            "ultra-new-session-id",
            "ultra-hash",
            "ultra-old-session-id",
        )
        .await;

    assert_eq!(
        got,
        ResponsePayload {
            result_code: ResultCode::Ok,
            payload: Some(UserInfo {
                access_level: AccessLevel::Client,
                user_id: "".to_owned()
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
            .path("/v1/auth/by/pin")
            .query_param("sessionid", "ultra-new-session-id")
            .query_param("deviceId", "ultra-device-id")
            .body("pinHash=ultra-hash&oldSessionId=ultra-old-session-id&auth_type=pin");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    make_client(&server)
        .auth_by_pin(
            "ultra-device-id",
            "ultra-new-session-id",
            "ultra-hash",
            "ultra-old-session-id",
        )
        .await;

    mock.assert()
}
