use crate::data_structs::*;

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
        self.request("/v1/ping", &[("sessionid", session_id)], &[])
            .await
            .json()
            .await
            .unwrap()
    }

    pub async fn request_session(&self) -> ResponsePayload<Session> {
        self.request("/v1/auth/session", &[], &[])
            .await
            .json()
            .await
            .unwrap()
    }

    pub async fn auth_by_phone(&self, session_id: &str, phone: &str) -> ResponsePayload<Nothing> {
        self.request(
            "/v1/auth/by/phone",
            &[("sessionid", session_id)],
            &[("phone", phone)],
        )
        .await
        .json()
        .await
        .unwrap()
    }

    pub async fn confirm_auth_by_phone(
        &self,
        session_id: &str,
        operation_ticket: &str,
        sms_code: &str,
    ) -> ResponsePayload<UserInfo> {
        self.request(
            "/v1/confirm",
            &[("sessionid", session_id)],
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

    pub async fn auth_by_password(
        &self,
        session_id: &str,
        password: &str,
    ) -> ResponsePayload<UserInfo> {
        self.request(
            "/v1/auth/by/password",
            &[("sessionid", session_id)],
            &[("password", password)],
        )
        .await
        .json()
        .await
        .unwrap()
    }

    pub async fn set_auth_pin(&self, session_id: &str, pin_hash: &str) -> ResponsePayload<Nothing> {
        self.request(
            "/v1/auth/pin/set",
            &[("sessionid", session_id)],
            &[("pinHash", pin_hash)],
        )
        .await
        .json()
        .await
        .unwrap()
    }

    pub async fn auth_by_pin(
        &self,
        session_id: &str,
        pin_hash: &str,
        old_session_id: &str,
    ) -> ResponsePayload<UserInfo> {
        self.request(
            "/v1/auth/by/pin",
            &[("sessionid", session_id)],
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

    pub async fn list_accounts(&self, session_id: &str) -> ResponsePayload<Vec<Account>> {
        self.request("/v1/accounts_flat", &[("sessionid", session_id)], &[])
            .await
            .json()
            .await
            .unwrap()
    }

    pub async fn list_operations(
        &self,
        session_id: &str,
        account_id: &str,
        start: u64,
        end: u64,
    ) -> ResponsePayload<Vec<Operation>> {
        let start = start.to_string();
        let end = end.to_string();

        self.request(
            "/v1/operations",
            &[("sessionid", session_id)],
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
            .query(&[("deviceId", &self.device_id)])
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

        make_client(&server).request("/example", &[], &[]).await;

        mock.assert()
    }

    #[rstest]
    #[tokio::test]
    async fn request_passes_device_id_also(server: MockServer) {
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/example")
                .query_param("appVersion", "5.5.1")
                .query_param("connectionSubtype", "4G")
                .query_param("appName", "mobile")
                .query_param("origin", "mobile,ib5,loyalty,platform")
                .query_param("connectionType", "Cellular")
                .query_param("platform", "android")
                .query_param("deviceId", "sample-device-id");
            then.status(200);
        });

        make_client(&server).request("/example", &[], &[]).await;

        mock.assert()
    }

    #[rstest]
    #[tokio::test]
    async fn request_passes_query_params_also_if_provided(server: MockServer) {
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
    async fn request_passes_form_also_if_provided(server: MockServer) {
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
