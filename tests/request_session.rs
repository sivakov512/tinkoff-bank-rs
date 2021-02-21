use httpmock::MockServer;
use rstest::*;
use tinkoff_bank_rs::{Client, ResponsePayload, ResultCode, Session};

const RESPONSE: &str = "{\"payload\": {\"sessionid\": \"session-id-example\", \"ttl\": 9994}, \"resultCode\": \"OK\", \"trackingId\": \"AZAZA11\"}";

#[fixture]
fn server() -> MockServer {
    MockServer::start()
}

#[rstest]
#[tokio::test]
async fn returns_session_details(server: MockServer) {
    server.mock(|when, then| {
        when.method(httpmock::Method::POST).path("/v1/auth/session");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });
    let client = Client::new(&server.base_url());

    let got = client.request_session().await;

    assert_eq!(
        got,
        ResponsePayload {
            result_code: ResultCode::Ok,
            payload: Session {
                id: "session-id-example".to_owned(),
                ttl: 9994
            }
        }
    )
}
