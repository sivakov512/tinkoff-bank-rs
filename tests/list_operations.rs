use httpmock::MockServer;
use rstest::*;
use tinkoff_bank_rs::{
    Category, Client, ClientBuilder, Currency, Merchant, MoneyAmount, Operation, OperationGroup,
    OperationTime, OperationType, ResponsePayload, ResultCode, Subgroup,
};

#[fixture]
fn server() -> MockServer {
    MockServer::start()
}

fn make_client(server: &MockServer) -> Client {
    ClientBuilder::default()
        .with_url(&server.base_url())
        .build()
}

#[rstest(
    response,
    expected,
    case(RESPONSE_1, Operation {
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
        subcategory: None,
        account: "100".to_owned(),
        merchant: Some(Merchant {
            name: "Яндекс.Еда".to_owned()
        }),
        group: OperationGroup::Pay,
        subgroup: None,
    }),
    case(RESPONSE_2, Operation {
        id: "1234567891".to_owned(),
        operation_type: OperationType::Debit,
        description: "Онлайм".to_owned(),
        amount: MoneyAmount {
            currency: Currency {
                code: 643,
                name: "RUB".to_owned()
            },
            value: 100.0
        },
        operation_time: OperationTime {
            milliseconds: 1613168606000
        },
        spending_category: Category {
            name: "Интернет".to_owned(),
        },
        mcc: 2,
        category: Category {
            name: "Интернет, voip/иб".to_owned(),
        },
        subcategory: Some("Онлайм".to_owned()),
        account: "100".to_owned(),
        merchant: None,
        group: OperationGroup::Pay,
        subgroup: Some(Subgroup {
            name: "".to_owned(),
        }),
    }),
    case(RESPONSE_3, Operation {
        id: "1234567892".to_owned(),
        operation_type: OperationType::Credit,
        description: "Иванов И.".to_owned(),
        amount: MoneyAmount {
            currency: Currency {
                code: 643,
                name: "RUB".to_owned()
            },
            value: 9999.0
        },
        operation_time: OperationTime {
            milliseconds: 1612978599000
        },
        spending_category: Category {
            name: "Пополнения".to_owned(),
        },
        mcc: 0,
        category: Category {
            name: "Другое".to_owned(),
        },
        subcategory: Some("Иванов И.".to_owned()),
        account: "100".to_owned(),
        merchant: None,
        group: OperationGroup::Income,
        subgroup: Some(Subgroup {
            name: "Пополнение по номеру телефона".to_owned(),
        }),
    }),
)]
#[tokio::test]
async fn returns_operations(response: &str, expected: Operation, server: MockServer) {
    server.mock(|when, then| {
        when.method(httpmock::Method::POST).path("/v1/operations");
        then.status(200)
            .header("Content-Type", "applucation/json")
            .body(response);
    });

    let got = make_client(&server)
        .list_operations("ultra-session-id", "100", 1234567890123, 1234567990123)
        .await;

    assert_eq!(
        got,
        ResponsePayload {
            result_code: ResultCode::Ok,
            payload: Some(vec![expected]),
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
            .body(RESPONSE_1);
    });

    make_client(&server)
        .list_operations("ultra-session-id", "100", 1234567890123, 1234567990123)
        .await;

    mock.assert()
}

const RESPONSE_1: &str = "{
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

const RESPONSE_2: &str = "{
    \"payload\": [
        {
            \"payment\": {
                \"bankAccountId\": \"100\",
                \"paymentId\": \"100500\",
                \"providerGroupId\": \"Интернет\",
                \"paymentType\": \"Payment\",
                \"feeAmount\": {
                    \"currency\": {
                        \"code\": 643,
                        \"name\": \"RUB\",
                        \"strCode\": \"643\"
                    },
                    \"value\": 0.0
                },
                \"providerId\": \"rostelekom-prosto\",
                \"fieldsValues\": {\"account\": \"123654\"},
                \"cardNumber\": \"553612******3456\"
            },
            \"id\": \"1234567891\",
            \"type\": \"Debit\",
            \"subgroup\": {\"id\": \"A1\", \"name\": \"\"},
            \"description\": \"Онлайм\",
            \"amount\": {
                \"currency\": {\"code\": 643, \"name\": \"RUB\", \"strCode\": \"643\"},
                \"value\": 100.0
            },
            \"operationTime\": {\"milliseconds\": 1613168606000},
            \"subcategory\": \"Онлайм\",
            \"spendingCategory\": {
                \"id\": \"37\",
                \"name\": \"Интернет\",
                \"icon\": \"40\",
                \"parentId\": \"5\"
            },
            \"mcc\": 2,
            \"category\": {\"id\": \"40\", \"name\": \"Интернет, voip/иб\"},
            \"account\": \"100\",
            \"card\": \"123456789\",
            \"group\": \"PAY\",
            \"cardNumber\": \"553612******3456\"
        }
    ],
    \"details\": {\"hasNext\": false},
    \"resultCode\": \"OK\",
    \"trackingId\": \"AZAZA11\"
}";

const RESPONSE_3: &str = "{
    \"payload\": [
        {
            \"id\": \"1234567892\",
            \"message\": \"Перевод денежных средств\",
            \"type\": \"Credit\",
            \"subgroup\": {
                \"id\": \"C10\",
                \"name\": \"Пополнение по номеру телефона\"
            },
            \"description\": \"Иванов И.\",
            \"senderDetails\": \"Иванов И.\",
            \"amount\": {
                \"currency\": {\"code\": 643, \"name\": \"RUB\", \"strCode\": \"643\"},
                \"value\": 9999.0
            },
            \"operationTime\": {\"milliseconds\": 1612978599000},
            \"subcategory\": \"Иванов И.\",
            \"spendingCategory\": {
                \"id\": \"70\",
                \"name\": \"Пополнения\",
                \"icon\": \"33\"
            },
            \"mcc\": 0,
            \"category\": {\"id\": \"33\", \"name\": \"Другое\"},
            \"account\": \"100\",
            \"card\": \"123456789\",
            \"group\": \"INCOME\",
            \"cardNumber\": \"553612******3456\"
        }
    ],
    \"details\": {\"hasNext\": false},
    \"resultCode\": \"OK\",
    \"trackingId\": \"AZAZA11\"
}";
