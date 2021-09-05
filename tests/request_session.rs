use httpmock::MockServer;
use rstest::*;
use tinkoff_bank::{Client, ClientBuilder, ResponsePayload, ResultCode, Session};

const RESPONSE: &str = "{\"payload\": {\"sessionid\": \"session-id-example\", \"ttl\": 9994}, \"resultCode\": \"OK\", \"trackingId\": \"AZAZA11\"}";

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
async fn returns_session_details(server: MockServer) {
    server.mock(|when, then| {
        when.method(httpmock::Method::POST).path("/v1/auth/session");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    let got = make_client(&server).request_session().await;

    assert_eq!(
        got,
        ResponsePayload {
            result_code: ResultCode::Ok,
            payload: Some(Session {
                id: "session-id-example".to_owned(),
                ttl: 9994
            }),
            confirmations: None,
            initial_operation: None,
            operation_ticket: None,
        }
    )
}
