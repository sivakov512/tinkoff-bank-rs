#![allow(dead_code)]
use uuid::Uuid;

const API_URL: &str = "https://api.tinkoff.ru";

pub struct Client {
    base_url: String,
    device_id: String,
    client: reqwest::Client,
}

pub struct ClientBuilder {
    base_url: String,
    device_id: String,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            base_url: API_URL.to_owned(),
            device_id: Uuid::new_v4().to_string(),
        }
    }
}

impl ClientBuilder {
    pub fn with_url(self, url: &str) -> Self {
        ClientBuilder {
            base_url: url.to_owned(),
            ..self
        }
    }

    pub fn build(self) -> Client {
        Client {
            base_url: self.base_url,
            device_id: self.device_id,
            client: reqwest::Client::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_client_with_production_url_by_default() {
        let client = ClientBuilder::default().build();

        assert_eq!(client.base_url, API_URL);
    }

    #[test]
    fn can_create_client_with_custom_url() {
        let client = ClientBuilder::default().with_url("http://lol.kek").build();

        assert_eq!(client.base_url, "http://lol.kek");
    }
}
