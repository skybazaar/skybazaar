use anyhow::Result;

use reqwest::{
    ClientBuilder,
    header::{self, HeaderMap, HeaderName, HeaderValue},
};

use crate::hypixel::{auctions::AuctionsHandler, bazaar::BazaarHandler};

pub struct HypixelClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    pub bazaar: BazaarHandler,
    pub auctions: AuctionsHandler,
}

impl HypixelClient {
    pub fn new(api_key: &str, user_agent: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();

        let mut api_key_header = HeaderValue::from_str(api_key)?;
        api_key_header.set_sensitive(true);

        headers.insert("API-Key", api_key_header);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .user_agent(user_agent)
            .build()?;

        Ok(Self {
            client: client.clone(),
            base_url: "https://api.hypixel.net/v2/skyblock".to_string(),
            api_key: api_key.to_string(),
            bazaar: BazaarHandler::new(client.clone()),
            auctions: AuctionsHandler::new(client),
        })
    }
}
