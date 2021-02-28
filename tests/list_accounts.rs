use httpmock::MockServer;
use rstest::*;
use tinkoff_bank_rs::{
    Account, Client, ClientBuilder, Currency, MoneyAmount, ResponsePayload, ResultCode,
};

const RESPONSE: &str = "{\"payload\": [{\"externalAccountNumber\": \"100000\", \"accountGroup\": \"Дебетовые карты\", \"moneyAmount\": {\"currency\": {\"code\": 643, \"name\": \"RUB\", \"strCode\": \"643\"}, \"value\": 1111.11}, \"currency\": {\"code\": 643, \"name\": \"RUB\", \"strCode\": \"643\"}, \"name\": \"Счет Tinkoff Black BE\", \"id\": \"100\"}, {\"externalAccountNumber\": \"200000\", \"accountGroup\": \"Дебетовые карты\", \"moneyAmount\": {\"currency\": {\"code\": 840, \"name\": \"USD\", \"strCode\": \"840\"}, \"value\": 22222.2}, \"currency\": {\"code\": 840, \"name\": \"USD\", \"strCode\": \"840\"}, \"name\": \"Счет USD Tinkoff Black\", \"id\": \"200\"}, {\"externalAccountNumber\": \"300000\", \"accountGroup\": \"Накопительные счета\", \"moneyAmount\": {\"currency\": {\"code\": 643, \"name\": \"RUB\", \"strCode\": \"643\"}, \"value\": 333333}, \"currency\": {\"code\": 643, \"name\": \"RUB\", \"strCode\": \"643\"}, \"name\": \"Классный счет\", \"id\": \"300\"}], \"details\": {\"hasNext\": false}, \"resultCode\": \"OK\", \"trackingId\": \"AZAZA11\"}";

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
async fn returns_accounts(server: MockServer) {
    server.mock(|when, then| {
        when.method(httpmock::Method::POST)
            .path("/v1/accounts_flat");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    let got = make_client(&server).list_accounts("ultra-session-id").await;

    assert_eq!(
        got,
        ResponsePayload {
            result_code: ResultCode::Ok,
            payload: Some(vec![
                Account {
                    external_number: "100000".to_owned(),
                    group: "Дебетовые карты".to_owned(),
                    money_amount: MoneyAmount {
                        currency: Currency::RUB,
                        value: 1111.11
                    },
                    name: "Счет Tinkoff Black BE".to_owned(),
                    id: "100".to_owned()
                },
                Account {
                    external_number: "200000".to_owned(),
                    group: "Дебетовые карты".to_owned(),
                    money_amount: MoneyAmount {
                        currency: Currency::USD,
                        value: 22222.2
                    },
                    name: "Счет USD Tinkoff Black".to_owned(),
                    id: "200".to_owned()
                },
                Account {
                    external_number: "300000".to_owned(),
                    group: "Накопительные счета".to_owned(),
                    money_amount: MoneyAmount {
                        currency: Currency::RUB,
                        value: 333333.0
                    },
                    name: "Классный счет".to_owned(),
                    id: "300".to_owned()
                }
            ]),
            confirmations: None,
            initial_operation: None,
            operation_ticket: None,
        }
    )
}

#[rstest]
#[tokio::test]
async fn passes_session_id(server: MockServer) {
    let mock = server.mock(|when, then| {
        when.method(httpmock::Method::POST)
            .path("/v1/accounts_flat")
            .query_param("sessionid", "ultra-session-id");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    make_client(&server).list_accounts("ultra-session-id").await;

    mock.assert()
}
