use crate::Client;
use uuid::Uuid;

const API_URL: &str = "https://api.tinkoff.ru";

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
    pub fn with_device_id(self, device_id: &str) -> Self {
        ClientBuilder {
            device_id: device_id.to_owned(),
            ..self
        }
    }

    pub fn build(self) -> Client {
        Client::new(self.base_url, self.device_id)
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
    fn creates_client_with_random_device_id_by_default() {
        let client1 = ClientBuilder::default().build();
        let client2 = ClientBuilder::default().build();

        assert_ne!(client1.device_id, client2.device_id);
    }

    #[test]
    fn can_create_client_with_custom_url() {
        let client = ClientBuilder::default().with_url("http://lol.kek").build();

        assert_eq!(client.base_url, "http://lol.kek");
    }

    #[test]
    fn can_create_client_with_custom_device_id() {
        let client = ClientBuilder::default().with_device_id("lol-kek").build();

        assert_eq!(client.device_id, "lol-kek");
    }

    #[test]
    fn customize_everything() {
        let client = ClientBuilder::default()
            .with_url("http://lol.kek")
            .with_device_id("lol-kek")
            .build();

        assert_eq!(client.base_url, "http://lol.kek");
        assert_eq!(client.device_id, "lol-kek");
    }
}
