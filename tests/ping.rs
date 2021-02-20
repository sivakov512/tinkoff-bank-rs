use httpmock::MockServer;
use rstest::*;
use tinkoff_bank_rs::{AccessLevel, Client, ResponsePayload, ResultCode, UserInfo};

#[fixture]
fn server() -> MockServer {
    MockServer::start()
}

#[rstest]
#[tokio::test]
async fn returns_anonymous_user(server: MockServer) {
    let _body = "{\"resultCode\": \"OK\", \"payload\": {\"accessLevel\": \"ANONYMOUS\", \"unreadMessagesCount\": 0, \"userId\": \"1111\"}, \"trackingId\": \"AZAZA11\"}";
    server.mock(|when, then| {
        when.method(httpmock::Method::POST).path("/v1/ping");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(_body);
    });
    let client = Client::new(&server.base_url());

    let got = client.ping().await;

    assert_eq!(
        got,
        ResponsePayload {
            result_code: ResultCode::Ok,
            payload: UserInfo {
                access_level: AccessLevel::Anonymous,
                user_id: "1111".to_owned()
            }
        }
    )
}
