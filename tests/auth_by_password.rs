use httpmock::MockServer;
use rstest::*;
use tinkoff_bank_rs::{AccessLevel, Client, ClientBuilder, ResponsePayload, ResultCode, UserInfo};

const RESPONSE: &str = "{\"payload\": {\"accessLevel\": \"CLIENT\", \"ssoId\": \"sso-id-example\", \"userId\": \"user-id-example\"}, \"resultCode\": \"OK\", \"trackingId\": \"AZAZA11\"} ";

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
        when.method(httpmock::Method::POST)
            .path("/v1/auth/by/password");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    let got = make_client(&server)
        .auth_by_password("ultra-session-id", "ultra-password")
        .await;

    assert_eq!(
        got,
        ResponsePayload {
            result_code: ResultCode::Ok,
            payload: Some(UserInfo {
                access_level: AccessLevel::Client,
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
            .path("/v1/auth/by/password")
            .query_param("sessionid", "ultra-session-id")
            .body("password=ultra-password");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    make_client(&server)
        .auth_by_password("ultra-session-id", "ultra-password")
        .await;

    mock.assert()
}