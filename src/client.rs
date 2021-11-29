use crate::data_structs::*;
use chrono::{DateTime, Utc};

const API_URL: &str = "https://api.tinkoff.ru";
const DEFAULT_PARAMS: [(&str, &str); 6] = [
    ("appVersion", "5.5.1"),
    ("connectionSubtype", "4G"),
    ("appName", "mobile"),
    ("origin", "mobile,ib5,loyalty,platform"),
    ("connectionType", "Cellular"),
    ("platform", "android"),
    // pass device id too
];

#[derive(Clone)]
pub struct Client {
    pub(crate) base_url: String,
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(API_URL)
    }
}

/// Client for Tinkoff bank API that is used by their mobile app.
///
/// In most cases you don't need to use `::new` method, so instantiate client with `::default`.
impl Client {
    /// Creates new `Client` with specified API url.

    /// Useful only for testing or working through proxy (maybe).
    pub fn new(base_url: &str) -> Self {
        Client {
            base_url: base_url.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    /// Ping bank API for details about specified session and device id.
    pub async fn ping(&self, device_id: &str, session_id: &str) -> ResponsePayload<UserInfo> {
        self.request(
            "/v1/ping",
            &[("deviceId", device_id), ("sessionid", session_id)],
            &[],
        )
        .await
        .json()
        .await
        .unwrap()
    }

    /// Ask bank API for new session.
    ///
    /// This method is a first call in any new interaction with bank API, so you should pregenrate
    /// device id and keep it.
    pub async fn request_session(&self, device_id: &str) -> ResponsePayload<Session> {
        self.request("/v1/auth/session", &[("deviceId", device_id)], &[])
            .await
            .json()
            .await
            .unwrap()
    }

    /// Start auth by phone.
    ///
    /// If successful case API returns operation ticket and user will get SMS code. All this
    /// details are needed to confirm auth in future.
    pub async fn auth_by_phone(
        &self,
        device_id: &str,
        session_id: &str,
        phone: &str,
    ) -> ResponsePayload<Nothing> {
        self.request(
            "/v1/auth/by/phone",
            &[("deviceId", device_id), ("sessionid", session_id)],
            &[("phone", phone)],
        )
        .await
        .json()
        .await
        .unwrap()
    }

    /// Complete auth by phone.
    ///
    /// Returns details on current session.
    pub async fn confirm_auth_by_phone(
        &self,
        device_id: &str,
        session_id: &str,
        operation_ticket: &str,
        sms_code: &str,
    ) -> ResponsePayload<UserInfo> {
        self.request(
            "/v1/confirm",
            &[("deviceId", device_id), ("sessionid", session_id)],
            &[
                ("initialOperationTicket", operation_ticket),
                ("initialOperation", "auth/by/phone"),
                (
                    "confirmationData",
                    &serde_json::json!({ "SMSBYID": sms_code }).to_string(),
                ),
            ],
        )
        .await
        .json()
        .await
        .unwrap()
    }

    /// Auth by password.
    ///
    /// You can't skip this step if you want to get full access to API. So, before calling this
    /// method you should complete auth by phone.
    pub async fn auth_by_password(
        &self,
        device_id: &str,
        session_id: &str,
        password: &str,
    ) -> ResponsePayload<UserInfo> {
        self.request(
            "/v1/auth/by/password",
            &[("deviceId", device_id), ("sessionid", session_id)],
            &[("password", password)],
        )
        .await
        .json()
        .await
        .unwrap()
    }

    /// Set auth pin like a mobile app does it with pin and fingerprint.
    ///
    /// Just generate pin hash and remember its value forever. In future you can use this hash to
    /// auth faster.
    pub async fn set_auth_pin(
        &self,
        device_id: &str,
        session_id: &str,
        pin_hash: &str,
    ) -> ResponsePayload<Nothing> {
        self.request(
            "/v1/auth/pin/set",
            &[("deviceId", device_id), ("sessionid", session_id)],
            &[("pinHash", pin_hash)],
        )
        .await
        .json()
        .await
        .unwrap()
    }

    /// Auth by pin like a mobile app does it with pin and fingerprint.
    ///
    /// To use you should know your previous (maybe outdated) session id. Request new session and
    /// provider both sessions (old and new) with pin hash, now new session has full access like
    /// old session previously.
    pub async fn auth_by_pin(
        &self,
        device_id: &str,
        session_id: &str,
        pin_hash: &str,
        old_session_id: &str,
    ) -> ResponsePayload<UserInfo> {
        self.request(
            "/v1/auth/by/pin",
            &[("deviceId", device_id), ("sessionid", session_id)],
            &[
                ("pinHash", pin_hash),
                ("oldSessionId", old_session_id),
                ("auth_type", "pin"),
            ],
        )
        .await
        .json()
        .await
        .unwrap()
    }

    /// List all bank accounts as flat list.
    ///
    /// Note that this method won't return any accounts from investing.
    pub async fn list_accounts(
        &self,
        device_id: &str,
        session_id: &str,
    ) -> ResponsePayload<Vec<Account>> {
        self.request(
            "/v1/accounts_flat",
            &[("deviceId", device_id), ("sessionid", session_id)],
            &[],
        )
        .await
        .json()
        .await
        .unwrap()
    }

    /// List operations for specified account id.
    ///
    /// Provide 'internal' account id, not account number! Real API doesn't require two dates for
    /// filtering, but I will.
    pub async fn list_operations(
        &self,
        device_id: &str,
        session_id: &str,
        account_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> ResponsePayload<Vec<Operation>> {
        let start = start.timestamp_millis().to_string();
        let end = end.timestamp_millis().to_string();

        self.request(
            "/v1/operations",
            &[("deviceId", device_id), ("sessionid", session_id)],
            &[("account", account_id), ("start", &start), ("end", &end)],
        )
        .await
        .json()
        .await
        .unwrap()
    }

    async fn request(
        &self,
        uri: &str,
        query: &[(&str, &str)],
        form: &[(&str, &str)],
    ) -> reqwest::Response {
        self.client
            .post(&format!("{}{}", self.base_url, uri))
            .query(&DEFAULT_PARAMS)
            .query(query)
            .form(form)
            .send()
            .await
            .unwrap()
    }
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
        Client::new(&server.base_url())
    }

    #[test]
    fn creates_client_with_specified_params() {
        let client = Client::new("http://lol.kek");

        assert_eq!(client.base_url, "http://lol.kek");
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

        make_client(&server).request("/example", &[], &[]).await;

        mock.assert()
    }

    #[rstest]
    #[tokio::test]
    async fn request_passes_query_params_if_provided(server: MockServer) {
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
            .request("/example", &[("key1", "val1"), ("key2", "val2")], &[])
            .await;

        mock.assert()
    }

    #[rstest]
    #[tokio::test]
    async fn request_passes_also_if_provided(server: MockServer) {
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
                .body("key1=val1&key2=val2");
            then.status(200);
        });

        make_client(&server)
            .request("/example", &[], &[("key1", "val1"), ("key2", "val2")])
            .await;

        mock.assert()
    }
}
