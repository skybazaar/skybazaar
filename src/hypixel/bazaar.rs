pub struct BazaarHandler {
    client: reqwest::Client,
}

impl BazaarHandler {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    // TODO: add the methods here
    // e.g. pub async fn get_items() { ... }
}
