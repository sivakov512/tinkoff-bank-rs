#![allow(dead_code)]
use serde::Deserialize;

const DEFAULT_PARAMS: [(&str, &str); 6] = [
    ("appVersion", "5.5.1"),
    ("connectionSubtype", "4G"),
    ("appName", "mobile"),
    ("origin", "mobile,ib5,loyalty,platform"),
    ("connectionType", "Cellular"),
    ("platform", "android"),
    // pass device id too
];

pub struct Client {
    pub(crate) base_url: String,
    pub(crate) device_id: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(base_url: String, device_id: String) -> Self {
        Client {
            base_url,
            device_id,
            client: reqwest::Client::new(),
        }
    }

    pub async fn ping(&self, session_id: &str) -> ResponsePayload<UserInfo> {
        self.request("/v1/ping", &[("sessionid", session_id)])
            .await
            .json()
            .await
            .unwrap()
    }

    pub async fn request_session(&self) -> ResponsePayload<Session> {
        self.request("/v1/auth/session", &[])
            .await
            .json()
            .await
            .unwrap()
    }

    async fn request(&self, uri: &str, query_params: &[(&str, &str)]) -> reqwest::Response {
        self.client
            .post(&format!("{}{}", self.base_url, uri))
            .query(&DEFAULT_PARAMS)
            .query(query_params)
            .send()
            .await
            .unwrap()
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
pub struct Session {
    #[serde(rename = "sessionid")]
    pub id: String,
    pub ttl: u32,
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
    pub payload: Option<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use rstest::*;

    #[fixture]
    fn server() -> MockServer {
        MockServer::start()
    }

    fn make_client(server: &MockServer) -> Client {
        Client::new(server.base_url(), "sample-device-id".to_owned())
    }

    #[test]
    fn creates_client_with_specified_params() {
        let client = Client::new("http://lol.kek".to_owned(), "lol-kek".to_owned());

        assert_eq!(client.base_url, "http://lol.kek");
        assert_eq!(client.device_id, "lol-kek");
    }

    #[rstest]
    #[tokio::test]
    async fn request_passes_default_params(server: MockServer) {
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/example")
                .query_param("appVersion", "5.5.1")
                .query_param("connectionSubtype", "4G")
                .query_param("appName", "mobile")
                .query_param("origin", "mobile,ib5,loyalty,platform")
                .query_param("connectionType", "Cellular")
                .query_param("platform", "android");
            then.status(200);
        });

        make_client(&server).request("/example", &[]).await;

        mock.assert()
    }

    #[rstest]
    #[tokio::test]
    async fn request_passes_extra_params_also_if_provided(server: MockServer) {
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/example")
                .query_param("appVersion", "5.5.1")
                .query_param("connectionSubtype", "4G")
                .query_param("appName", "mobile")
                .query_param("origin", "mobile,ib5,loyalty,platform")
                .query_param("connectionType", "Cellular")
                .query_param("platform", "android")
                // additional params
                .query_param("key1", "val1")
                .query_param("key2", "val2");
            then.status(200);
        });

        make_client(&server)
            .request("/example", &[("key1", "val1"), ("key2", "val2")])
            .await;

        mock.assert()
    }
}
