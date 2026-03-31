pub struct AuctionsHandler {
    client: reqwest::Client,
}

impl AuctionsHandler {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    // TODO: add the methods here
    // e.g. pub async fn get_items() { ... }
}
