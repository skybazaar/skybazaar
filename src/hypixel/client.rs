use reqwest::Client;

pub struct HypixelClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl HypixelClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.hypixel.net/v2/skyblock".to_string(),
            api_key,
        }
    }
}
