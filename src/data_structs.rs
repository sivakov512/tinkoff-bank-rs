use serde::{Deserialize, Deserializer};

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

#[derive(Debug, PartialEq)]
pub enum Currency {
    RUB,
    USD,
}

impl<'de> Deserialize<'de> for Currency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Outer {
            name: Inner,
        }

        #[derive(Deserialize)]
        enum Inner {
            RUB,
            USD,
        }

        let helper = Outer::deserialize(deserializer)?;
        Ok(match helper.name {
            Inner::RUB => Currency::RUB,
            Inner::USD => Currency::USD,
        })
    }
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

#[derive(Debug, PartialEq)]
pub struct Operation {
    pub id: String,
    pub operation_type: OperationType,
    pub description: String,
    pub amount: MoneyAmount,
    pub operation_time: OperationTime,
    pub spending_category: String,
    pub mcc: u16,
    pub category: String,
    pub subcategory: Option<String>,
    pub account: String,
    pub merchant: Option<String>,
    pub group: OperationGroup,
    pub subgroup: Option<String>,
}

impl<'de> Deserialize<'de> for Operation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Outer {
            id: String,
            #[serde(rename = "type")]
            operation_type: OperationType,
            description: String,
            amount: MoneyAmount,
            #[serde(rename = "operationTime")]
            operation_time: OperationTime,
            #[serde(rename = "spendingCategory")]
            spending_category: InnerName,
            mcc: u16,
            category: InnerName,
            subcategory: Option<String>,
            account: String,
            merchant: Option<InnerName>,
            group: OperationGroup,
            subgroup: Option<InnerName>,
        }

        #[derive(Deserialize)]
        struct InnerName {
            name: String,
        }

        let helper = Outer::deserialize(deserializer)?;
        Ok(Operation {
            id: helper.id,
            operation_type: helper.operation_type,
            description: helper.description,
            amount: helper.amount,
            operation_time: helper.operation_time,
            spending_category: helper.spending_category.name,
            mcc: helper.mcc,
            category: helper.category.name,
            subcategory: helper.subcategory,
            account: helper.account,
            merchant: match helper.merchant {
                Some(val) => Some(val.name),
                None => None,
            },
            group: helper.group,
            subgroup: match helper.subgroup {
                Some(val) => Some(val.name),
                None => None,
            },
        })
    }
}
