use anyhow::{Result, anyhow};

use crate::hypixel::auctions::models::AuctionsPageResponse;

pub struct AuctionsHandler {
    client: reqwest::Client,
    base_url: reqwest::Url,
}

impl AuctionsHandler {
    pub fn new(client: reqwest::Client, base_url: reqwest::Url) -> Self {
        Self { client, base_url }
    }

    pub async fn fetch_page(&self, page: u32) -> Result<AuctionsPageResponse> {
        let url = self.base_url.join("auctions")?;

        let response = self
            .client
            .get(url)
            .query(&[("page", page)])
            .send()
            .await?
            .error_for_status()?
            .json::<AuctionsPageResponse>()
            .await?;

        if !response.success {
            return Err(anyhow!("Hypixel returned success=false for page {}", page));
        }

        Ok(response)
    }
}
