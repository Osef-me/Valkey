use reqwest::Client;

pub struct Api {
    client: Client,
    base_url: String,
}

impl Api {
    pub fn new(base_url: String, bearer_token: String) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", bearer_token).parse().unwrap(),
        );
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client, base_url }
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }
}
