use httpmock::MockServer;
use rstest::*;
use tinkoff_bank::{AccessLevel, Client, ClientBuilder, ResponsePayload, ResultCode, UserInfo};

const ANONYMOUS: &str = "{\"resultCode\": \"OK\", \"payload\": {\"accessLevel\": \"ANONYMOUS\", \"unreadMessagesCount\": 0, \"userId\": \"1111\"}, \"trackingId\": \"AZAZA11\"}";
const CANDIDATE: &str = "{\"resultCode\": \"OK\", \"payload\": {\"ssoId\": \"100-500-azaza-lolkek\", \"accessLevel\": \"CANDIDATE\", \"additionalAuth\": {\"needLogin\": false, \"needPassword\": true, \"needRegister\": false}, \"unreadMessagesCount\": 0, \"userId\": \"1234\"}, \"trackingId\": \"AZAZA11\"}";
const CLIENT: &str = "{\"resultCode\": \"OK\", \"payload\": {\"ssoId\": \"100-500-azaza-lolkek\", \"accessLevel\": \"CLIENT\", \"unreadMessagesCount\": 0, \"userId\": \"1234\"}, \"trackingId\": \"AZAZA11\"}";

#[fixture]
fn server() -> MockServer {
    MockServer::start()
}

fn make_client(server: &MockServer) -> Client {
    ClientBuilder::default()
        .with_url(&server.base_url())
        .build()
}

#[rstest(resp, expected,
    case(
        ANONYMOUS,
        UserInfo {
            access_level: AccessLevel::Anonymous,
            user_id: "1111".to_owned(),
        },
    ),
    case(
        CANDIDATE,
        UserInfo {
            access_level: AccessLevel::Candidate,
            user_id: "1234".to_owned(),
        },
    ),
    case(
        CLIENT,
        UserInfo {
            access_level: AccessLevel::Client,
            user_id: "1234".to_owned(),
        },
    ),
)]
#[tokio::test]
async fn returns_user_details(resp: &str, expected: UserInfo, server: MockServer) {
    server.mock(|when, then| {
        when.method(httpmock::Method::POST).path("/v1/ping");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(resp);
    });

    let got = make_client(&server).ping("ultra-session-id").await;

    assert_eq!(
        got,
        ResponsePayload {
            result_code: ResultCode::Ok,
            payload: Some(expected),
            confirmations: None,
            initial_operation: None,
            operation_ticket: None,
        }
    )
}

#[rstest]
#[tokio::test]
async fn passes_session_id_as_query(server: MockServer) {
    let mock = server.mock(|when, then| {
        when.method(httpmock::Method::POST)
            .path("/v1/ping")
            .query_param("sessionid", "ultra-session-id");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(CLIENT);
    });

    make_client(&server).ping("ultra-session-id").await;

    mock.assert()
}
