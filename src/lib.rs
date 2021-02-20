use serde::Deserialize;

const API_URL: &str = "https://api.tinkoff.ru";

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Client::new(API_URL)
    }
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Client {
            base_url: base_url.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn ping(&self) -> ResponsePayload<UserInfo> {
        let resp = self
            .client
            .post(&format!("{}/v1/ping", self.base_url))
            .send()
            .await
            .unwrap();
        resp.json().await.unwrap()
    }
}

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
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum ResultCode {
    #[serde(rename = "OK")]
    Ok,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ResponsePayload<T> {
    #[serde(rename = "resultCode")]
    pub result_code: ResultCode,
    pub payload: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_predefined_api_by_default() {
        let client = Client::default();

        assert_eq!(client.base_url, "https://api.tinkoff.ru");
    }
}
