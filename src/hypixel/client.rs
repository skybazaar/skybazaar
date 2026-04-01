use anyhow::Result;

use reqwest::{
    ClientBuilder, Url,
    header::{HeaderMap, HeaderValue},
};
use serde::Deserialize;

use crate::hypixel::{auctions::AuctionsHandler, bazaar::handler::BazaarHandler};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HypixelResponse<T> {
    pub success: bool,
    pub last_updated: u64,
    #[serde(alias = "products")]
    pub data: T,
}

pub struct HypixelClient {
    pub bazaar: BazaarHandler,
    pub auctions: AuctionsHandler,
}

impl HypixelClient {
    pub fn new(api_key: Option<&str>, user_agent: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();

        match api_key {
            Some(key) => {
                let mut api_key_header = HeaderValue::from_str(key)?;
                api_key_header.set_sensitive(true);

                headers.insert("API-Key", api_key_header);
            }
            None => (),
        }

        let client = ClientBuilder::new()
            .default_headers(headers)
            .user_agent(user_agent)
            .build()?;

        let base_url = Url::parse("https://api.hypixel.net/v2/skyblock/")?;

        Ok(Self {
            bazaar: BazaarHandler::new(client.clone(), base_url.clone()),
            auctions: AuctionsHandler::new(client, base_url),
        })
    }
}
