use httpmock::MockServer;
use rstest::*;
use tinkoff_bank_rs::{
    Category, Client, ClientBuilder, Currency, Merchant, MoneyAmount, Operation, OperationGroup,
    OperationTime, OperationType, ResponsePayload, ResultCode,
};

const RESPONSE: &str = "{
    \"payload\": [
        {
            \"id\": \"1234567890\",
            \"type\": \"Credit\",
            \"authMessage\": \"Операция утверждена.\",
            \"description\": \"Яндекс.Еда\",
            \"amount\": {
                \"currency\": {\"code\": 643, \"name\": \"RUB\", \"strCode\": \"643\"},
                \"value\": 1234.50
            },
            \"operationTime\": {\"milliseconds\": 1613639239000},
            \"spendingCategory\": {
                \"id\": \"24\",
                \"name\": \"Рестораны\",
                \"icon\": \"32\",
                \"parentId\": \"3\"
            },
            \"mcc\": 5812,
            \"category\": {\"id\": \"32\", \"name\": \"Рестораны\"},
            \"account\": \"100\",
            \"merchant\": {
                \"name\": \"Яндекс.Еда\",
                \"region\": {\"country\": \"RUS\", \"city\": \"MOSKVA\"}
            },
            \"card\": \"123456789\",
            \"group\": \"PAY\",
            \"cardNumber\": \"553612******3456\"
        }
    ],
    \"details\": {\"hasNext\": false},
    \"resultCode\": \"OK\",
    \"trackingId\": \"AZAZA11\"
}";

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
async fn returns_operations(server: MockServer) {
    server.mock(|when, then| {
        when.method(httpmock::Method::POST).path("/v1/operations");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    let got = make_client(&server)
        .list_operations("ultra-session-id", "100", 1234567890123, 1234567990123)
        .await;

    assert_eq!(
        got,
        ResponsePayload {
            result_code: ResultCode::Ok,
            payload: Some(vec![Operation {
                id: "1234567890".to_owned(),
                operation_type: OperationType::Credit,
                description: "Яндекс.Еда".to_owned(),
                amount: MoneyAmount {
                    currency: Currency {
                        code: 643,
                        name: "RUB".to_owned()
                    },
                    value: 1234.5
                },
                operation_time: OperationTime {
                    milliseconds: 1613639239000
                },
                spending_category: Category {
                    name: "Рестораны".to_owned(),
                },
                mcc: 5812,
                category: Category {
                    name: "Рестораны".to_owned(),
                },
                account: "100".to_owned(),
                merchant: Merchant {
                    name: "Яндекс.Еда".to_owned()
                },
                group: OperationGroup::Pay,
            },]),
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
            .path("/v1/operations")
            .query_param("sessionid", "ultra-session-id")
            .body("account=100&start=1234567890123&end=1234567990123");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(RESPONSE);
    });

    make_client(&server)
        .list_operations("ultra-session-id", "100", 1234567890123, 1234567990123)
        .await;

    mock.assert()
}
