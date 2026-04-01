pub struct AuctionsHandler {
    client: reqwest::Client,
    base_url: reqwest::Url,
}

impl AuctionsHandler {
    pub fn new(client: reqwest::Client, base_url: reqwest::Url) -> Self {
        Self { client, base_url }
    }

    // TODO: add the methods here
    // e.g. pub async fn get_items() { ... }
}
