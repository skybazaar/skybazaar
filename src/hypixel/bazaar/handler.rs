use crate::hypixel::bazaar::models::BazaarData;
use anyhow::{Result, anyhow};

pub struct BazaarHandler {
    client: reqwest::Client,
    base_url: reqwest::Url,
}

impl BazaarHandler {
    pub fn new(client: reqwest::Client, base_url: reqwest::Url) -> Self {
        Self { client, base_url }
    }
    pub async fn fetch(&self) -> Result<BazaarData> {
        let response = self
            .client
            .get(self.base_url.join("bazaar")?)
            .send()
            .await?
            .error_for_status()?
            .json::<BazaarData>()
            .await?;

        if !response.success {
            return Err(anyhow!("Hypixel responded with success = false"));
        }

        Ok(response)
    }
}
