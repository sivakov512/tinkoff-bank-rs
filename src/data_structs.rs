use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum AccessLevel {
    #[serde(rename = "ANONYMOUS")]
    Anonymous,
    #[serde(rename = "CANDIDATE")]
    Candidate,
    #[serde(rename = "CLIENT")]
    Client,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct UserInfo {
    #[serde(rename = "accessLevel")]
    pub access_level: AccessLevel,
    #[serde(default, rename = "userId")]
    pub user_id: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Session {
    #[serde(rename = "sessionid")]
    pub id: String,
    pub ttl: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum ResultCode {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "WAITING_CONFIRMATION")]
    WaitingConfirmation,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Nothing {}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Account {
    #[serde(rename = "externalAccountNumber")]
    pub external_number: String,
    #[serde(rename = "accountGroup")]
    pub group: String,
    #[serde(rename = "moneyAmount")]
    pub money_amount: MoneyAmount,
    pub name: String,
    pub id: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct MoneyAmount {
    pub currency: Currency,
    pub value: f32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Currency {
    pub code: u32,
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ResponsePayload<T> {
    #[serde(rename = "resultCode")]
    pub result_code: ResultCode,
    // exists for success response
    pub payload: Option<T>,
    // exists if confirmation required
    pub confirmations: Option<Vec<String>>,
    #[serde(rename = "initialOperation")]
    pub initial_operation: Option<String>,
    #[serde(rename = "operationTicket")]
    pub operation_ticket: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum OperationType {
    Credit,
    Debit,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct OperationTime {
    pub milliseconds: u64,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Category {
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Merchant {
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Subgroup {
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum OperationGroup {
    #[serde(rename = "PAY")]
    Pay,
    #[serde(rename = "INCOME")]
    Income,
    #[serde(rename = "TRANSFER")]
    Transfer,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CORRECTION")]
    Correction,
    #[serde(rename = "CHARGE")]
    Charge,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Operation {
    pub id: String,
    #[serde(rename = "type")]
    pub operation_type: OperationType,
    pub description: String,
    pub amount: MoneyAmount,
    #[serde(rename = "operationTime")]
    pub operation_time: OperationTime,
    #[serde(rename = "spendingCategory")]
    pub spending_category: Category,
    pub mcc: u16,
    pub category: Category,
    pub subcategory: Option<String>,
    pub account: String,
    pub merchant: Option<Merchant>,
    pub group: OperationGroup,
    pub subgroup: Option<Subgroup>,
}
